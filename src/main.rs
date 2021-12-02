// Language: Rust

/*
    Blochchain :
    - A blockchain is a list of blocks.
    - Each block contains a list of transactions.

    A block contains :
    - Previous hash
    - Current hash
    - A list of transactions.

    Transaction contains :
    - User 1 address
    - User 2 address
    - Amount of money
    - Signature of the transaction


    Proof of work :
    - A proof of work is a hash that starts with a specific number of zeros.
    - The goal is to find a hash that starts with a specific number of zeros.
    - The number of zeros is called the difficulty.
    - The difficulty is increased by one every time a block is mined.
    - The difficulty is decreased by one every time a block is rejected.


    Proof of stake :
    - A proof of stake is a hash that starts with a specific number of zeros.
    - The goal is to find a hash that starts with a specific number of zeros.
    - The number of zeros is called the difficulty.
    - The difficulty is increased by one every time a block is mined.
    - The difficulty is decreased by one every time a block is rejected.

    How transactions are verified :
    - The transaction is signed by the user.
    - The signature is verified by the user.

    How blocks are verified :
    - The previous hash is verified by the user.
    - The current hash is verified by the user.
    - The list of transactions is verified by the user.

    How blockchain transaction works:
    - The user sends money to another user.
    - The user signs the transaction.
    - The user sends the transaction to the blockchain.
    - The blockchain adds the transaction to the list of transactions.

    How blockchain mining works:
    - The user sends money to another user.
    - The user signs the transaction.
    - The user sends the transaction to the blockchain.
    - The blockchain adds the transaction to the list of transactions.
    - The blockchain mines a block.
    - The blockchain adds the block to the list of blocks.
    - The blockchain sends the block to the user.
    - The user verifies the block.
    - The user verifies the transaction.
    - The user verifies the signature.
    - The user verifies the previous hash.
    - The user verifies the current hash.
    - The user verifies the list of transactions.

    Blockchain algorithm :

    init_blockchain() {
        genesis_block = create_genesis_block();
        blockchain = [genesis_block];
    }

    create_genesis_block() {
        return {
            "index": 0,
            "timestamp": "",
            "transactions": [],
            "proof": "",
            "previous_hash": "",
            "current_hash": ""
        };
    }

    create_block(previous_hash) {
        block = {
            "index": len(blockchain) + 1,
            "timestamp": "",
            "transactions": [],
            "proof": "",
            "previous_hash": previous_hash,
            "current_hash": ""
        };
        return block;
    }

    blockchain_add_block(block) {
        block["current_hash"] = calculate_hash(block);
        blockchain.append(block);
    }

    calculate_hash(block) {
        block_string = json.dumps(block, sort_keys=True).encode();
        return hashlib.sha256(block_string).hexdigest();
    }

    is_valid_proof(transactions, previous_hash, proof) {
        guess_hash = calculate_hash(transactions, previous_hash, proof);
        return guess_hash[:difficulty] == "0" * difficulty;
    }

    proof_of_work(transactions, previous_hash) {
        proof = 0;
        while not is_valid_proof(transactions, previous_hash, proof):
            proof += 1;
        return proof;
    }

    is_valid_block(block, previous_hash) {
        return block["previous_hash"] == previous_hash and block["current_hash"] == calculate_hash(block);
    }

    is_valid_chain(blockchain) {
        previous_hash = "0";
        for block in blockchain:
            if not is_valid_block(block, previous_hash):
                return False;
            previous_hash = block["current_hash"];
        return True;
    }

    add_transaction(transaction) {
        transactions.append(transaction);
        return index(transactions);
    }

    mine_block(transactions, previous_hash) {
        proof = proof_of_work(transactions, previous_hash);
        block = create_block(previous_hash, proof);
        blockchain_add_block(block);
        return block;
    }

    get_balance(address) {
        balance = 0;
        for block in blockchain:
            for transaction in block["transactions"]:
                if transaction["from"] == address:
                    balance -= transaction["amount"];
                if transaction["to"] == address:
                    balance += transaction["amount"];
        return balance;
    }

    get_last_block() {
        return blockchain[-1];
    }

    is_valid_transaction(transaction) {
        return get_balance(transaction["from"]) >= transaction["amount"];
    }
 */

mod scytale;
mod rot13;
mod cesar;
mod playfair;

fn main() {
    scytale::run("MXTTUEFEQEAEEBSEOIVRERSANULNRCOOLNEAERUORTEECALINATUUSSNIDEDTHVASDHE");
    println!("\n\n");
    rot13::run("OBAWBHE OBO, WR FRENV QVFCBAVOYR YHAQV, IRARM OBVER DHRYDHR GRR, FBYHGVBAARE QRF RKBF QR PELCGBTENCUVR");
    rot13::run("FNYHG NYVPR, WHFGR CBHE GR QVER DHR EBG13 A'RFG CNF HA PVCURE GERF SBEG.");
    println!("\n\n");
    cesar::run("Vcfgrwqwoizcuwgtowbsobhgoizcuwgsghqseisqsghoixcifrviwxcifrstshsqcaasbhsghqseisjcigbsgojsndogeishobhrsgofhwgobgjcigbsrsjsndogjcigacbhfsfibxcifcijfwsfgobgojcwfzsgwbgwubsgrsjcgdfctsggwcbgdofzshcweiszsghhcbashwsf");
    println!("\n\n");
    playfair::run("sur la doua villeurbanne", "CPELYON", 'W');
}