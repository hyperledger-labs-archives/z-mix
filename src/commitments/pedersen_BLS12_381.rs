use commitments::CommitmentScheme;

struct PedersenCommitmentBLS12381 {}

impl CommitmentScheme for PedersenCommitmentBLS12381 {
    // Returns `num_elements` + 1 generators. This is useful when committing to several messages say
    // m_1, m_2, m_3, and so on. `setup` will this output g_1, g_2, g_3, g_4 and so on which can then be
    // used for commitment f(g_1, g_2, g_3, g_4, ..., m_1, m_2, m_3...)
    fn setup(num_elements: u32) -> Vec<Vec<u8>> {
        unimplemented!();
    }

    fn commit(generators: &[&[u8]], messages: &[&[u8]]) -> Vec<u8> {
        unimplemented!();
    }

    fn verify(commitment: &[u8], opening: &[u8], generators: &[&[u8]], messages: &[&[u8]]) -> bool {
        unimplemented!();
    }
}

// Alternate implementation

/*
impl<Group> CommitmentScheme for PedersenCommitment {
    fn setup(num_elements: u32) -> Vec<Group::Element> {
        unimplemented!();
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_one_element() {
        let result = PedersenCommitment::setup(1);
        assert_eq!(result.len(), 1);
    }
}
