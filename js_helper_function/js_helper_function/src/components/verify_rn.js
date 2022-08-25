import crypto from "crypto";
import { decodePlatformState, decodeGameState } from "./game_platform_state";

export const getRandomNoFromHash = (hashedValue) => {
    let index = 0;
    let result;
  
    do {
      result = parseInt(hashedValue.substring(index * 5, index * 5 + 5), 16);
      index += 1;
      if (index * 5 + 5 > 129) {
        result = 9999;
        break;
      }
    } while (result >= 1e6);
    return result % 6;
  };

export const verify_rn = async () => {
    const platform_state = await decodePlatformState();
    const game_state = await decodeGameState();

    const server_seed = platform_state.server_seed.toString();
    const client_seed = game_state.client_seed.toString();
    const nonce = game_state.nonce.toString();

    const combination = client_seed + server_seed + nonce;

    const hash = crypto.createHash("sha256").update(combination).digest("hex");

    console.log("Client Seed:", client_seed);
    console.log("Server Seed:", server_seed);
    console.log("Nonce:", nonce);
    console.log("Hash:", hash);
    
    const random_number = getRandomNoFromHash(hash);
    
    console.log("Random Number", random_number);
}