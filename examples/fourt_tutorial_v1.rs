
use halo2_proofs::{
     circuit::{Layouter, SimpleFloorPlanner},
    dev::MockProver,
    plonk::{Advice, Circuit, Column,TableColumn, ConstraintSystem, Error,    Selector},
    poly::Rotation,
};
 use halo2_proofs::plonk::Expression;
use pairing::bn256::Fr as Fp;

fn main() {
    const K: u32 = 6;

    #[derive(Clone)]
    struct FaultyCircuitConfig {
        a: Column<Advice>,
        q: Selector,
        table: TableColumn,
    }

    struct FaultyCircuit {}

    impl Circuit<Fp> for FaultyCircuit {
        type Config = FaultyCircuitConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            let a = meta.advice_column();
            let q = meta.complex_selector();
            let table = meta.lookup_table_column();

            meta.lookup("lookup", |cells| {
                let a = cells.query_advice(a, Rotation::cur());
                let q = cells.query_selector(q);

                // If q is enabled, a must be in the table.
                // When q is not enabled, lookup the default value instead.
                let not_q = Expression::Constant(Fp::one()) - q.clone();
                let default = Expression::Constant(Fp::from(2));
                vec![(q * a + not_q * default, table)]
            });

            FaultyCircuitConfig { a, q, table }
        }

        fn without_witnesses(&self) -> Self {
            Self {}
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<Fp>,
        ) -> Result<(), Error> {
            layouter.assign_table(
                || "Doubling table",
                |mut table| {
                    (1..(1 << (K - 1)))
                        .map(|i| {
                            println!("The {} value is added in the table",2*i);
                            table.assign_cell(
                                || format!("table[{}] = {}", i, 2 * i),
                                config.table,
                                i - 1,
                                || Ok(Fp::from(2 * i as u64)),
                            )
                        })
                        .fold(Ok(()), |acc, res| acc.and(res))
                },
            )?;

            layouter.assign_region(
                || "Good synthesis",
                |mut region| {
                    // Enable the lookup on rows 0 and 1.
                    config.q.enable(&mut region, 0)?;
                    config.q.enable(&mut region, 1)?;

                    println!("{} is checking",2);
                    // Assign a = 2 and a = 6.
                    region.assign_advice(|| "a = 2", config.a, 0, || Ok(Fp::from(2)))?;
                    println!("{} is checking",6);
                    region.assign_advice(|| "a = 6", config.a, 1, || Ok(Fp::from(6)))?;

                    Ok(())
                },
            )?;

            layouter.assign_region(
                || "Faulty synthesis",
                |mut region| {
                    // Enable the lookup on rows 0 and 1.
                    config.q.enable(&mut region, 0)?;
                    config.q.enable(&mut region, 1)?;

                    println!("{} is checking",7);
                    // Assign a = 7.
                    region.assign_advice(|| "a = 7", config.a, 0, || Ok(Fp::from(7)))?;
                    println!("{} is checking",5);
                    // BUG: Assign a = 5, which doesn't exist in the table!
                    region.assign_advice(|| "a = 5", config.a, 1, || Ok(Fp::from(6)))?;
                     Ok(())
                },
            )
        }
    }

    let prover = MockProver::run(K, &FaultyCircuit {}, vec![]).unwrap();
    //assert!(prover.verify().is_err());
    assert_eq!(prover.verify(), Ok(()));
    /*assert_eq!(
        prover.verify(),
        Err(vec![VerifyFailure::Lookup {
            name: "lookup",
            lookup_index: 0,
            location: FailureLocation::InRegion {
                region: (2, "Faulty synthesis").into(),
                offset: 1,
            }
        }])
    );*/
}
