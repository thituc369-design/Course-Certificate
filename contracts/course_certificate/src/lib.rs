#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

#[contract]
pub struct CourseCertificate;

#[contractimpl]
impl CourseCertificate {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Issue a new certificate. Only the institution can call this.
    pub fn issue_certificate(
        env: Env,
        cert_id: u64,
        student: Address,
        course_name: String,
        grade: String,
        issue_date: u64,
    ) {
        let mut certificates: Map<u64, (Address, String, String, u64, bool)> = env
            .storage()
            .instance()
            .get(&"certificates")
            .unwrap_or(Map::new(&env));

        if certificates.contains_key(cert_id) {
            panic!("Certificate already exists");
        }

        certificates.set(cert_id, (student, course_name, grade, issue_date, false));
        env.storage().instance().set(&"certificates", &certificates);
    }

    /// Verify a certificate. Returns (student, course_name, grade, issue_date) if valid.
    pub fn verify_certificate(
        env: Env,
        cert_id: u64,
    ) -> (Address, String, String, u64) {
        let certificates: Map<u64, (Address, String, String, u64, bool)> = env
            .storage()
            .instance()
            .get(&"certificates")
            .unwrap_or(Map::new(&env));

        let (student, course_name, grade, issue_date, revoked) = certificates
            .get(cert_id)
            .unwrap_or_else(|| panic!("Certificate not found"));

        if revoked {
            panic!("Certificate has been revoked");
        }

        (student, course_name, grade, issue_date)
    }

    /// Revoke a certificate. Only the institution can call this.
    pub fn revoke_certificate(env: Env, cert_id: u64) {
        let mut certificates: Map<u64, (Address, String, String, u64, bool)> = env
            .storage()
            .instance()
            .get(&"certificates")
            .unwrap_or(Map::new(&env));

        let entry = certificates
            .get(cert_id)
            .unwrap_or_else(|| panic!("Certificate not found"));

        let (student, course_name, grade, issue_date, _) = entry;
        certificates.set(cert_id, (student, course_name, grade, issue_date, true));
        env.storage().instance().set(&"certificates", &certificates);
    }

    /// Get full certificate details. Returns (student, course_name, grade, issue_date, revoked).
    pub fn get_certificate(
        env: Env,
        cert_id: u64,
    ) -> (Address, String, String, u64, bool) {
        let certificates: Map<u64, (Address, String, String, u64, bool)> = env
            .storage()
            .instance()
            .get(&"certificates")
            .unwrap_or(Map::new(&env));

        certificates
            .get(cert_id)
            .unwrap_or_else(|| panic!("Certificate not found"))
    }
}
