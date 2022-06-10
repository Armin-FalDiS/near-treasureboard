<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/armin-faldis/near-treasureboard">
    <img src="https://user-images.githubusercontent.com/20294274/172386153-0a3aa6f2-1357-40f8-89c2-a6dab311f303.png" alt="Logo" width="160" height="180">
  </a>

<h3 align="center">NEAR - TreasureBoard</h3>

  <p align="center">
    Near TreasureBoard is a game which has a user create a treasure board putting a prize in it. Then, players can reserve a slot on the board by paying 1 NEAR. After half the slots on the board have been reserved, the game is closed which allows the creator to reveal the position of the bombs on the board and initiate a prize payout.
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#the-cli">The CLI</a></li>
        <ul>
          <li><a href="#prerequisites">Prerequisites</a></li>
          <li><a href="#installation">Installation</a></li>
          <li><a href="#usage">Usage</a></li>
        </ul>
        <li><a href="#the-smart-contract">The Smart Contract</a></li>
        <ul>
          <li><a href="#contract-prerequisites">Prerequisites</a></li>
          <li><a href="#contract-installation">Installation</a></li>
          <li><a href="#contract-usage">Usage</a></li>
          <li><a href="#notes-and-remarks">Notes and Remarks</a></li>
        </ul>
      </ul>
    </li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

A game starts when a creator chooses a size and creates a new treasure board. The creator has to deposit NEAR tokens proportional to the size of the treasure board which are Small(4), Medium(16) and Big(36). The creator also has to provide the hash of the solution which is the slots that contain a bomb. Thus, half the slots on the treasureboard have bombs in them. <br>
The remaining half of the treasureboard contains a random piece of the treasure (creator's deposit) in each of it's slots. Each player has to deposit 1 NEAR to reserve a slot on the treasureboard. After half of the slots have been reserved, the treasureboard closes
which allow the creator to reveal the solution.<br>
The hash of the revealed solution has to match the originally provided hash upon game creation. During a successful reveal, each answer is checked and if an answer triggers a bomb, the player loses their 1 NEAR to the creator of the game. On the other hand, if a player chose a slot that was not bombed, they get a random cut of the treasure. It is possible to have all or most of the treasure in one slot
or a slot with no treasure in it but this will not be known since it is calculated after the revelation.

<p align="right">(<a href="#top">back to top</a>)</p>



### Built With

* [Rust](https://www.rust-lang.org/)
* [NearSDK](https://www.near-sdk.io/)
* [NEAR](https://near.org/)
* [Node.js](https://nodejs.org/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This project has a backend which is  a NEAR smart contract written in Rust using near-sdk-rs.<br>
There is also a CLI included that is written in NodeJS using near-api-js.<br><br>

The Command Line Interface can be used to communicate with the smart contract deployed on the blockchain with ease.<br>

### The CLI

The simple CLI can be found in the "interface" directory but to use it, you have to first create a wallet on NEAR's testnet. This can be achieved with the help of near-cli.


#### Prerequisites

First and foremost, you have to install NodeJS. Do note that you have to install npm along with it so be sure to have npm selected when you see it on NodeJS's installation wizard. You can get NodeJS from [here](https://nodejs.org/en/).<br>
After that, you have to install near-cli. This can be achieved by running this one line in your command prompt / terminal.<br>
```
npm install -g near-cli
```
Now that you have near-cli up and running, you have to login to a test account wallet. This is way easier than it sounds and can be done by running this command: <br>
```
  near login
```
By running that command, your browser will open up taking you to a user friendly website allowing you to create your wallet. This is very straight forward. After you have created your wallet, simply enter the id (name) of the created account in the already opened terminal.<br>
Congrats ! Now you have a wallet on NEAR testnet. Do remember your account Id since you will need it later.

#### Installation

1. Clone the repo (Or download zip from github)
   ```
   git clone https://github.com/armin-faldis/near-treasureboard
   ```
2. Open a terminal in the "interface" directory
3. Install NPM packages
   ```
   npm install
   ```
4. Now you can run the CLI
   ```
   node .
   ```

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
#### Usage

This is a very simple and straight forward CLI which if you ran successfully should show you a list of actions that you can do by entering their corresponding number. The interface will prompt you to enter information in a step-by-step manner and inform you of any errors.<br>
Happy treasureboarding !!

<p align="right">(<a href="#top">back to top</a>)</p>

### The Smart Contract

The smart contract which can be found in the "contract" directory is already deployed to the testnet and the CLI can be utilized to use it so there is no need to redeploy it.


#### Contract Prerequisites

To get into the smart contract, you can use rustup which is a installer making it easy to get started with rust. It can can downloaded from [here](https://www.rust-lang.org/learn/get-started). You would also need cargo which is a package(crate?) manager included with rustup.

#### Contract Installation

1. Clone the repo (Or download zip from github)
   ```
   git clone https://github.com/armin-faldis/near-treasureboard
   ```
2. Open a terminal in the "contract" directory
3. Install cargo packages
   ```
   cargo update
   ```
5. Add the WebAssembly toolchain if you already haven't
   ```
   rustup target add wasm32-unknown-unknown
   ```
6. Done. You can now use cargo to build or test the contract
   ```
   cargo build
   cargo test
   ```
7. To generate a wasm file which can be used to deploy to the blockchain use the provided build files
   ```
   // Windows
   build.bat
   // Linux
   ./build.sh
   ```
   
<!-- USAGE EXAMPLES -->
#### Contract Usage

The smart contract can be executed either by unit tests (cargo test) or deploying the generated wasm file to the testnet and using the provided CLI.

#### Notes and Remarks

The smart contract locks NEARs equal to the number of slots on the board which makes creators pay for the storage cost of their game's information by the very prize they deposit upon game creation. This essentially charges players for the staked storage and refunds them (by the prize payout) when the game is finished and the information is removed from the state freeing the storage stake. As such, a seperate mechanism to charge for storage stake is entirely unnecessary.<br>
Furthermore, the economical logic of this game is as sound as it could be (without KYC). No attempts were made in any part of the code to prevent an account from playing more than once at any given game since obviously they could just make another account. A player can create a game and reserve all the slots then reveal the answer getting their tokens back burning gas on every transaction or they could reserve half the playable slots which they know are not bombed but since treasure distribution is random they could end up losing all their tokens anyways.<br>
All in all, given the fact that there are no possible sure fire ways for someone to always make money in this game coupled with the solution checking mechanism using a hash function makes this game fair to both the creator and the players. 


<p align="right">(<a href="#top">back to top</a>)</p>
