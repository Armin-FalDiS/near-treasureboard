import * as nearAPI from 'near-api-js';
import { sha256 } from 'js-sha256';
import { homedir } from 'os';
import { join } from 'path';
import * as inp from 'readline-sync';


// init keystore (from .near-credentials)
const CREDENTIALS_DIR = '.near-credentials';
const credentialsPath = join(homedir(), CREDENTIALS_DIR);
const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(
	credentialsPath,
);

/**
 * use SHA256 to hash the given array
 * @param {number[]} arr
 * @returns SHA256 hashed array
 */
function hash_solution(arr) {
	return sha256.array(arr);
}

/**
 * Connect to the Near testnet
 * @param {string} accountId The account id used to sign transactions
 * @returns {nearAPI.Contract} The contract on the testnet
 */
async function initNear(accountId) {
	// connect to near testnet
	const near = await nearAPI.connect({
		networkId: 'testnet',
		keyStore: keyStore,
		nodeUrl: 'https://rpc.testnet.near.org',
		walletUrl: 'https://wallet.testnet.near.org',
		helperUrl: 'https://helper.testnet.near.org',
		explorerUrl: 'https://explorer.testnet.near.org',
	});

	// load account
	const account = await near.account(accountId);

	// load contract
	const contract = new nearAPI.Contract(
		account,
		'dev-1654580401237-76489858696902',
		{
			viewMethods: ['games'],
			changeMethods: ['new_game', 'play', 'reveal'],
		},
	);

	return contract;
}

/**
 * Fetch all treasureboards
 * @param {string} accountId Id of the account to sign the transaction with
 * @returns {{id: number, size: string, answers: number[]}}
 */
async function games(accountId) {
	let contract = await initNear(accountId);

	return await contract.games({
		args: {},
		gas: 40 * 1e12,
	});
}

/**
 * Creates a new treasureboard game
 * @param {string} accountId Id of the account to sign the transaction with
 * @param {string} size Size of the treasure board (Small, Medium, Big)
 * @param {number[]} bombs Placement of the bombs on the board (should contain salt after bomb places)
 * @returns outcome of the functioncall excecution
 */
async function newGame(accountId, size, bombs) {
	let contract = await initNear(accountId);

	// determine size
	let size_num;
	switch (size) {
		case 'Small':
			size_num = 4;
			break;
		case 'Medium':
			size_num = 16;
			break;
		case 'Big':
			size_num = 32;
			break;
		default:
			size_num = 0;
	}

	if (size_num == 0) {
		throw new Error('Given size is invalid');
	}
	if (bombs.length() < size / 2) {
		throw new Error('Bombs array must match the size');
	}

	// hash the solution
	bombs = hash_solution(bombs);

	return await contract.new_game({
		args: {
			size: size,
			solution_hash: bombs,
		},
		gas: 40 * 1e12,
		amount: size_num,
	});
}

/**
 * Play on a treasure board reserving the choice
 * @param {string} accountId Id of the account to sign the transaction with
 * @param {number} id Id of the treasure board
 * @param {number} choice The slot on the treasure board to reserve
 * @returns outcome of the functioncall excecution
 */
async function play(accountId, id, choice) {
	let contract = await initNear(accountId);

	return await contract.play({
		args: {
			id: id,
			choice: choice,
		},
		gas: 40 * 1e12,
		amount: 1,
	});
}

/**
 * Reveal the solution of a closed treasureboard bringing it to an end
 * Only the creator of a treasureboard can reveal the answer
 * @param {string} accountId Account id of the creator of the treasureboard
 * @param {number} id  Id of the treasureboard
 * @param {number[]} solution Plain array which it's hash was given upon creating the game
 * @returns outcome of the functioncall excecution
 */
async function reveal(accountId, id, solution) {
	let contract = await initNear(accountId);

	return await contract.play({
		args: {
			id: id,
			solution: solution,
		},
		gas: 40 * 1e12,
	});
}

console.log('!!!\t\tWelcome to NEAR-TreasureBoard by Armin FalDiS\t\t!!!');
console.log('###\t\tPlease login using near-cli before using this tool\t\t###');

while(true) {
	console.log(`
	Available actions are as followed:
	\t1. Start a new game
	\t2. Get the list of games
	\t3. Play a game
	\t4. Reveal the solution of a game
	\t0. Exit
	\n`);

	let action = Number(inp.question('Pick your poison: '));

	if (action == 0) {
		break;
	}

	let accountId = inp.question(
		'Enter your accountId (you should have already logged-in with near-cli): ',
	);

	switch (action) {
		case 1:
			let size = inp.question('Choose a size for this game (Small, Medium, Big): ');
			size = size.toLowerCase();
			if (size != 'small' || size != 'medium' || size != 'big') {
				console.error('\nThat was an invalid size. Defaulting to Small.');
				size = 'small';
			}
			size = size[0].toUpperCase() + size.substring(1);

			let bombs = inp.question(
				'Now enter slots that contain bomb followed by some random numbers (numbers between 0-255 seperated by SPACE): ',
			);
			bombs = bombs.split(' ').map((b, i) => {
				try {
					let bomb = Number(b);
					if (bomb < 0 || bomb > 255) throw new Error('Invalid u8');
					return bomb;
				} catch (err) {
					console.error('\nInvalid number given. Defaulting to index');
					return i;
				}
			});

			try {
				console.log(await newGame(accountId, size, bombs));
			} catch (err) {
				console.error(err);
			}
			break;
		case 2:
			try {
				console.log(await games(accountId));
			} catch (err) {
				console.error(err);
			}
			break;
		case 3:
			let play_id = inp.question('Enter the id of the game: ');
			let choice = inp.question('Which slot do you wish to choose? ');

			try {
				play_id = Number(play_id);
				choice = Number(choice);
			} catch (err) {
				console.error('\nThat was not a valid input !');
			}

			if (!isNaN(play_id) && !isNaN(choice)) {
				try {
					console.log(await play(accountId, play_id, choice));
				} catch (err) {
					console.error(err);
				}
			}
			break;
		case 4:
			let reveal_id = inp.question('Enter the id of the game: ');
			let solution = inp.question(
				'Now enter solution (The numbers provided during game creation seperated by SPACE): ',
			);
			solution = solution.split(' ').map((b, i) => {
				try {
					let bomb = Number(b);
					if (bomb < 0 || bomb > 255) throw new Error('Invalid u8');
					return bomb;
				} catch (err) {
					console.error('\nInvalid number given. Defaulting to index');
					return i;
				}
			});

			if(!isNaN(reveal_id)) {
				try {
					console.log(await reveal(accountId, reveal_id, solution));
				} catch (err) {
					console.error(err);
				}
			}

			break;
		default:
			console.log("\nSorry, I didn't quite catch that !");
	}
}