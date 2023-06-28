use bellperson::{gadgets::num::AllocatedNum, ConstraintSystem, SynthesisError};
use nova_snark::{
	traits::{
		circuit::{StepCircuit, TrivialTestCircuit},
		Group,
	},
	CompressedSNARK, PublicParams,
};
use pasta_curves::group::ff;

type G1 = pasta_curves::pallas::Point;
type G2 = pasta_curves::vesta::Point;

#[derive(Clone)]
pub struct FibonacciCircuit<F: ff::PrimeField + Clone> {
	a: F,
	b: F,
}

impl<F> StepCircuit<F> for FibonacciCircuit<F>
where
	F: ff::PrimeField,
{
	fn arity(&self) -> usize {
		2
	}

	fn synthesize<CS: ConstraintSystem<F>>(
		&self,
		cs: &mut CS,
		z: &[AllocatedNum<F>],
	) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
		let sum = AllocatedNum::alloc(cs.namespace(|| "sum"), || Ok(self.a + self.b))?;

		// Enforce the constraint that sum = z[0] + z[1]
		cs.enforce(
			|| "sum constraint",
			|lc| lc + z[0].get_variable(),
			|lc| lc + z[1].get_variable(),
			|lc| lc + sum.get_variable(),
		);

		Ok(vec![z[1].clone(), sum])
	}

	fn output(&self, z: &[F]) -> Vec<F> {
		assert_eq!(z.len(), 2); // Ensure we have two inputs
		vec![z[0] + z[1]] // The output is the sum of the inputs
	}
}

fn main() {
	let circuit_primary =
		FibonacciCircuit { a: <G1 as Group>::Scalar::zero(), b: <G1 as Group>::Scalar::one() };

	let circuit_secondary = TrivialTestCircuit::<<G2 as Group>::Scalar>::default();

	let pp = PublicParams::<
		G1,
		G2,
		FibonacciCircuit<<G1 as Group>::Scalar>,
		TrivialTestCircuit<<G2 as Group>::Scalar>,
	>::setup(circuit_primary.clone(), circuit_secondary.clone());

	println!("Number of constraints per step (primary circuit): {}", pp.num_constraints().0);
	println!("Number of constraints per step (secondary circuit): {}", pp.num_constraints().1);

	println!("Number of variables per step (primary circuit): {}", pp.num_variables().0);
	println!("Number of variables per step (secondary circuit): {}", pp.num_variables().1);

	todo!()
}
