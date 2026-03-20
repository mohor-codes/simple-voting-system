# 🗳️ Soroban Voting Smart Contract

A simple decentralized voting smart contract built using the Soroban SDK (Rust) on the Stellar network.

---

## 🚀 Features

* Initialize election with candidates
* One vote per address
* Prevent double voting
* Store votes on-chain
* Fetch vote counts

---

## 📦 Project Structure

```
.
├── src/
│   └── lib.rs      # Smart contract code
├── Cargo.toml
└── README.md
```

---

## ⚙️ Prerequisites

Make sure you have:

* Rust installed → https://rustup.rs/
* Soroban CLI installed
* WASM target added

```bash
rustup target add wasm32-unknown-unknown
```

---

## 📥 Installation

Clone the repository:

```bash
git clone https://github.com/your-username/soroban-voting.git
cd soroban-voting
```

---

## 🔨 Build Contract

Compile to WASM:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Output file:

```
target/wasm32-unknown-unknown/release/voting_contract.wasm
```

---

## 🌐 Deploy Contract
CCNSSE2RFGZWBJOSGWDKYNCIURN5R4QMAHNDQK4K43TE45A43QAKK6WQ
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/voting_contract.wasm \
  --source YOUR_ACCOUNT
```

---

## 🧪 Usage

### 1. Initialize Candidates

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn init \
  --arg '[ "Alice", "Bob", "Charlie" ]'
```

---

### 2. Vote

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn vote \
  --source YOUR_ACCOUNT \
  --arg YOUR_ADDRESS \
  --arg "Alice"
```

---

### 3. Get Votes

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn get_votes \
  --arg "Alice"
```

---

## 🧠 How It Works

* Candidates are stored during initialization
* Each candidate has a vote counter
* Voters are tracked to prevent double voting
* Votes are permanently stored on-chain

---

## ⚠️ Limitations

* No admin control
* No time limit for voting
* Not optimized for large-scale elections

---

## 🔒 Security Notes

* Each voter must authenticate
* Double voting is prevented
* Contract should be audited before production use

---

## 📌 Future Improvements

* Add admin role
* Time-based voting
* DAO integration
* Token-weighted voting

---

## 🧑‍💻 Author

Your Name

---

## 📄 License

MIT License
# simple-voting-system
voting system
