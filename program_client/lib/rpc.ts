// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum NftInstruction {
/**
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
    Initialize = 0,

/**
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
    Vote = 1,

/**
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
    GetResults = 2,
}

export type InitializeArgs = {
    feePayer: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const initialize = (args: InitializeArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
            },
        },
        {
            id: NftInstruction.Initialize,
        }
    );


    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const initializeSendAndConfirm = async (
    args: Omit<InitializeArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(initialize({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

export type VoteArgs = {
    feePayer: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const vote = (args: VoteArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
            },
        },
        {
            id: NftInstruction.Vote,
        }
    );


    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const voteSendAndConfirm = async (
    args: Omit<VoteArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(vote({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

export type GetResultsArgs = {
    feePayer: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const getResults = (args: GetResultsArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
            },
        },
        {
            id: NftInstruction.GetResults,
        }
    );


    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 */
export const getResultsSendAndConfirm = async (
    args: Omit<GetResultsArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(getResults({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

// Getters

export const getVotingData = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.VotingData | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeVotingData(deserialize(T.VotingDataSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

