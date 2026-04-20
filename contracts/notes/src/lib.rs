#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data catatan keuangan
#[contracttype]
#[derive(Clone, Debug)]
pub struct FinanceNote {
    id: u64,
    title: String,
    amount: i64,      // jumlah uang
    note_type: String, // "income" / "expense"
}

// Storage key
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct FinanceContract;

#[contractimpl]
impl FinanceContract {

    // Ambil semua data
    pub fn get_notes(env: Env) -> Vec<FinanceNote> {
        env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah catatan keuangan
    pub fn create_note(
        env: Env,
        title: String,
        amount: i64,
        note_type: String,
    ) -> String {
        let mut notes: Vec<FinanceNote> = env
            .storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        let note = FinanceNote {
            id: env.prng().gen::<u64>(),
            title,
            amount,
            note_type,
        };

        notes.push_back(note);

        env.storage().instance().set(&NOTE_DATA, &notes);

        String::from_str(&env, "Catatan keuangan berhasil ditambahkan")
    }

    // Hapus catatan
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<FinanceNote> = env
            .storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);

                env.storage().instance().set(&NOTE_DATA, &notes);

                return String::from_str(&env, "Berhasil hapus data");
            }
        }

        String::from_str(&env, "Data tidak ditemukan")
    }
}