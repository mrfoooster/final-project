// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

export interface VotingData {
  option1Votes: number;
  option2Votes: number;
}

export const decodeVotingData = (decoded: Decoded): VotingData => ({
    option1Votes: decoded["option_1_votes"] as number,
    option2Votes: decoded["option_2_votes"] as number,
});

export const VotingDataSchema: Schema =  {
    struct: {
        option_1_votes: "u32",
        option_2_votes: "u32",
    }
};



