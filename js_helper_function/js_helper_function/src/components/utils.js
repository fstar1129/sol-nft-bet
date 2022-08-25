import * as BufferLayout from "buffer-layout";


       const publicKey = (property = "publicKey") => {
        return BufferLayout.blob(32, property);
      }
 const uint64 = (property = "uint64") => {
  return BufferLayout.blob(8, property);
};  


  export const MINT_LAYOUT = BufferLayout.struct([

    BufferLayout.u8("isInitialized"),
    publicKey("mint1"),
    publicKey("mint2"),
    publicKey("mint3"),
    publicKey("mint4"),
    publicKey("mint5"),
    publicKey("mint6"),

   
  ]);

