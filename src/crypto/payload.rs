pub struct CanonicalPayload;

impl CanonicalPayload {
    pub fn seed_payload(
        member_num: &str,
        name: &str,
        date: &str,
        authority_fingerprint: &str,
    ) -> String {
        format!(
            "org:dev:|seq:{}|name:{}|date:{}|authority:{}",
            member_num,
            name.trim(),
            date,
            authority_fingerprint
        )
    }

    pub fn final_payload(
        cred_id: &str,
        name: &str,
        date: &str,
        legality_id: &str,
        signatory_name: &str,
        signatory_role: &str,
    ) -> String {
        format!(
            "org:dev:|cred_id:{}|name:{}|date:{}|legality:{}|signatory:{}|role:{}",
            cred_id,
            name.trim(),
            date,
            legality_id,
            signatory_name,
            signatory_role
        )
    }
}
