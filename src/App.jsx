import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Web3 from "web3";
import dotenv from "dotenv";
import { Wallet } from "ethers";

function App() {
  const [ethEddress, setEthAddress] = useState("");
  const [ethWallet, setEthWallet] = useState("");
  const [ethBalance, setEthBalance] = useState("");
  const [newData, setNewData] = useState("");

  async function create_wallet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let invoke_create = await invoke("create_wallet");
    setEthAddress(invoke_create.public_address);
    setEthWallet(invoke_create);
  }

  async function get_balance() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let invoke_create = await invoke("get_balance", { wallet: ethWallet });
    setEthBalance(invoke_create);
  }

  useEffect(() => {
    const mnemonic =
      "broccoli immense lobster discover kitchen merge plastic hip broccoli fly medal supply";
    const mnemonicWallet = Wallet.fromPhrase(mnemonic);
    console.log(mnemonicWallet.address);
    console.log("2");
    let provider = import.meta.env.VITE_APP_ALCHEMY_KEY;
    let web3Provider = new Web3.providers.WebsocketProvider(provider);
    let web3 = new Web3(web3Provider);
    const subscription = web3.eth
      .subscribe("newBlockHeaders")
      .on("data", function (blockHeader) {
        get_balance();
      })
      .on("error", console.error);
    return () => subscription.unsubscribe();
  }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            create_wallet();
          }}
        >
          <button type="submit">Create Wallet</button>
        </form>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            get_balance();
          }}
        >
          <button type="submit">Get Balance</button>
        </form>
      </div>

      <p>{ethEddress}</p>
      <br></br>
      <p>{ethBalance}</p>
    </div>
  );
}

export default App;
