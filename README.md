# 💰 Stellar Finance DApp

**Stellar Finance DApp** - Decentralized Financial Note Application on the Stellar Blockchain

---

## 📌 Project Description

Stellar Finance DApp is a blockchain-based application used to record financial activities such as **income** and **expenses** in a decentralized way using **Soroban Smart Contracts** on the Stellar network.

This application allows users to:

* Store financial data securely on the blockchain
* Manage financial records without relying on centralized databases
* Access and control data transparently

All data is stored directly on the blockchain, which means:

* It cannot be tampered with
* It does not depend on any server
* It is more secure and transparent

---

## 🎯 Project Vision

The goals of this application are:

* 💡 To help users manage their finances in a simple way
* 🔐 To provide blockchain-based data security
* 🌐 To reduce dependency on centralized systems
* 📊 To make financial tracking more transparent

---

## ⚙️ Key Features

### 1. ✍️ Create Financial Note

* Add a new financial record
* Inputs:

  * Title (e.g., "Buy coffee")
  * Amount
  * Type: income / expense

---

### 2. 📄 Get All Notes

* Retrieve all financial records
* Data is fetched directly from the blockchain

---

### 3. ❌ Delete Note

* Delete a record by its ID
* The data will be permanently removed from contract storage

---

### 4. 💵 Balance Calculation (Additional Feature 🔥)

* Automatically calculates total balance:

  * Income ➜ added
  * Expense ➜ subtracted

---

## 🧠 Smart Contract Structure

```rust
pub struct FinanceNote {
    id: u64,
    title: String,
    amount: i64,
    note_type: String, // income / expense
}
```

---

## 📦 Tech Stack

* 🦀 Rust (Soroban Smart Contract)
* 🌐 Stellar Blockchain
* ⚡ Soroban SDK

---

## 🚀 How to Run

### 1. Build Contract

```bash
stellar contract build
```

### 2. Deploy Contract

```bash
stellar contract deploy --source-account YOUR_ACCOUNT
```

---

### 3. Interact with Contract

#### ➕ Create Note

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source-account YOUR_ACCOUNT \
-- create_note \
--title "Gaji" \
--amount 5000000 \
--note_type "income"
```

---

#### 📄 Get Notes

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source-account YOUR_ACCOUNT \
-- get_notes
```

---

#### ❌ Delete Note

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source-account YOUR_ACCOUNT \
-- delete_note \
--id NOTE_ID
```

---

## 📍 Contract Information

* Network: Stellar Testnet
* Contract ID: **CCS6AMVJAA56FQB3BGA7KKHQPVQFFDGODCYY2OOYDPOBFTPVX45IZEVQ**
* Wallet: Freighter / CLI

---

## 🔮 Future Improvements

* 📊 Financial statistics (charts)
* 🔍 Filter by date
* 🏷️ Categories (food, transport, etc.)
* 🔐 Data encryption
* 📱 Frontend integration (Flutter / Web)

---

## 📝 Submission Notes

* This project is an enhanced version of the workshop example (Notes DApp)
* Modified into a **financial tracking application**
* Includes an additional feature: **balance calculation**
* Built using Soroban Smart Contract on Stellar Testnet

---

## 📸 Screenshot



---

## 👨‍💻 Author

* Name: Mahfudh Al Rafif
* Major: Informatics Engineering
* University: Politeknik Negeri Jakarta (PNJ)

---

## ⭐ Conclusion

Stellar Finance DApp demonstrates how blockchain technology can be used to build a simple yet powerful application for managing financial data securely, transparently, and without relying on third parties.

---

💡 *Built with Soroban on Stellar*
