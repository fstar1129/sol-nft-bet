import React, { useState , useEffect} from 'react';
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { decodeMetadata, getMetadataAccount } from "./components/helper";

import { paltform_state } from './components/platform_state';
import { init_platform } from './components/Init_platform';
import { create_collection } from './components/cratecollection';
import { create_game } from './components/create_game';
import { join_game } from './components/join_game';
import { claim_nft } from './components/claim_nft';
import { print_states } from './components/game_platform_state';
import { disjoin_game } from './components/disjoin_game';
import { verify_rn } from './components/verify_rn';

import { user_nft_mint } from './components/ids';







const App = () => {
  const [count, setCount] = useState();
  const [pubKey, setPubKey] = useState();
  const [amount, setAmount] = useState();
  const [amount1, setAmount1] = useState();

  const [mint, setMint] = useState();
  const [nftObjData, setNftObjData] = useState();
  
  useEffect(() => {

  },[nftObjData]);




  /////////////////////////////////////////////////////////////Connections////////////////////////////////////////////    
  const getConnectedWallet = async()=> {    
    const provider = await window.solana;
    if(provider){
        setPubKey(provider.publicKey);
        localStorage.setItem("pubKey", provider.pubKey);
    }
    else console.log("Try to connect again");
    }


    const connectWallet = async() => {
        const provider = window.solana;
        console.log(provider);
        if(provider){
                setCount(count + 1);
                await window.solana.connect();
                window.solana.on("connect", () => console.log("connect"));
                getConnectedWallet();
            }
        else window.open("https://phantom.app/", "_blank")
    }

    const disconnectWallet = () => {
        window.solana.disconnect();
        localStorage.removeItem('pubKey')
        setPubKey();
    }

    const getNft = async (publicKey) => {
      console.log("working");
      let connection = new Connection(clusterApiUrl("devnet"), "confirmed");
      let response = await connection.getParsedTokenAccountsByOwner(publicKey, {
        programId: TOKEN_PROGRAM_ID,
      });
      let mints = await Promise.all(
        response.value
          .filter(
            (accInfo) => accInfo.account.data.parsed.info.tokenAmount.uiAmount !== 0
          )
          .map((accInfo) =>
            getMetadataAccount(accInfo.account.data.parsed.info.mint)
          )
      );
      let mintPubkeys = mints.map((m) => new PublicKey(m));
      let multipleAccounts = await connection.getMultipleAccountsInfo(mintPubkeys);
      let nftMetadata = multipleAccounts
        .filter((account) => account !== null)
        .map((account) => decodeMetadata(account.data));
      return nftMetadata;
    };
    
    const getNftData = async (publicKey) => {
      let nftData = await getNft(publicKey);
      console.log(nftData);
      let nftMintName = [];
    
      nftData.map(async (nft) => {
        let res = await fetch(nft.data.uri);
        let data = await res.json();
        let nftObj = {
          name: nft.data.name,
          mint: nft.mint,
          image: data.image,
        };
        nftMintName.push(nftObj);
        console.log(nftMintName);
        setNftObjData(nftMintName);
      });
    };



    const selectNft = (mintkey) =>{
      setMint(mintkey)
    }

    return (
      <div className = "App">
          <h1>Hey: { pubKey ? pubKey.toString() : ""}</h1>
          <br />
          <button onClick = {connectWallet}>Connect Here!</button>
          <button onClick = {disconnectWallet}>Disconnect Here!</button>
          <br/><br/>
          
        <label>Make offer (SOL): </label>
          <br/><br/>
        <button onClick = {() => getNftData(pubKey)}>get NFTs Data</button>

          <br/><br/>
        
        
          {nftObjData ? (
        <>
          <h3>NFTs!!</h3>
          <ol style={{ listStyle: "none", marginRight: "40px" }}>
            {nftObjData.map((nft) => (
              <li key={nft.mint}>
                <img src={nft.image} onClick={() => selectNft(nft.mint)} />
                <br />
                <br />
                <h5>mint: </h5>
                <a
                  key={nft.mint}
                  href={`https://explorer.solana.com/address/${nft.mint}?cluster=devnet`}
                >
                  {nft.mint}{" "}
                </a>
                <br />
                <br />
              </li>
            ))}
          </ol>
        </>
      ) : (
        <></>
      )}



        <input type="text" onChange = {(e) => setAmount((e.target.value))} /><h3>nonce</h3>
        <input type="text" onChange = {(e) => setAmount1((e.target.value))} /><h3>fees</h3>

        <br/><br/><br/><br/>
        <button onClick = {() => init_platform(pubKey,amount,amount1)}>Init_platform</button>

        <br/><br/><br/><br/>
        <button onClick = {() => create_collection(pubKey)}>create_collection</button>

        <br/><br/><br/><br/>
        <button onClick = {() => create_game(pubKey)}>create_game</button>

        <br/><br/><br/><br/>
        <button onClick = {() => join_game(pubKey,mint)}>join_game</button>

        <br/><br/><br/><br/>
        <button onClick = {() => disjoin_game(pubKey,user_nft_mint)}>leave_game</button>

        <br/><br/><br/><br/>
        <button onClick = {() => claim_nft(pubKey,mint)}>claim_nft</button>

        <br/><br/><br/><br/>
        <button onClick = {() => verify_rn(pubKey,mint)}>verify_rn</button>

        <br/><br/><br/><br/><br/>
        <button onClick={() => print_states()}>Print Game State</button>
      </div>
      ) 
  }
export default App
