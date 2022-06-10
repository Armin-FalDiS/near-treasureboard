import * as nearAPI from 'near-api-js';
import { sha256 } from 'js-sha256';
import { homedir } from 'os';
import { join } from 'path';
import * as input from 'readline-sync';

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
 * Validates treasureboard size and returns a valid value
 * Defaults to Small in case of wrong values
 * @param {string} size Input size
 * @returns {string} Validated size
 */
function validate_size(size) {
	size = size.toLowerCase();
	if (size != 'small' && size != 'medium' && size != 'big') {
		console.error('\nThat was an invalid size. Defaulting to Small.');
		size = 'small';
	}

	return size[0].toUpperCase() + size.substring(1);
}

/**
 * @param {string} size Size of the treasureboard
 * @returns {number} Total number of slots on the treasureboard
 */
function get_slots(size) {
	switch (size) {
		case 'Small':
			return 4;
		case 'Medium':
			return 16;
		case 'Big':
			return 32;
		default:
			return 0;
	}
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
		'treasure-board.armin-faldis.testnet',
		{
			viewMethods: ['games'],
			changeMethods: ['new_game', 'play', 'reveal'],
		},
	);

	return contract;
}

/**
 * Fetch all treasureboards
 * @returns {[{id: number, size: string, answers: number[]}]}
 */
async function games() {
	let contract = await initNear(accountId);

	return await contract.games({ args: {} });
}

/**
 * Creates a new treasureboard game
 * @param {string} accountId Id of the account to sign the transaction with
 * @param {string} size Size of the treasure board (Small, Medium, Big)
 * @param {string} bombs Placement of the bombs on the board (SPACE seperated numbers)
 * @param {string} salt String to add to the end of bombs securing the generated hash
 * @returns outcome of the functioncall excecution
 */
async function newGame(accountId, size, slots, bombs, salt) {
	let contract = await initNear(accountId);

	// determine the deposit in yoctoNEARs needed to create a game of this size
	let deposit_amount = slots + '000000000000000000000000';

	// turn bombs into number array
	bombs = bombs.split(' ').map((b, i) => {
		try {
			let bomb = Number(b);
			// check if bomb's slot is within bounds
			if (bomb < 0 || bomb >= slots) throw new Error('Invalid u8');
			return bomb;
		} catch (err) {
			console.error('\nInvalid number given. Defaulting to index');
			return i;
		}
	});

	// bombs should match the size of the board
	if (bombs.length < slots / 2) {
		throw new Error(
			'Number of bombs must match the size of the treasure board',
		);
	}

	// add salt to bombs as byte array
	let buffer = Buffer.from(salt);
	buffer.forEach((x) => bombs.push(x));

	// print plain solution for user
	console.log(
		'\n\nThis is the solution which can be used to reveal this treasureboard: \n',
	);
	console.log(bombs.reduce((p, c) => p + ' ' + c, ''));
	console.log('\nSave it somewhere\n\n');

	return await contract.new_game({
		args: {
			size: size,
			solution_hash: hash_solution(bombs),
		},
		gas: '40000000000000',
		amount: deposit_amount,
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
		gas: '40000000000000',
		amount: '1000000000000000000000000',
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

	return await contract.reveal({
		args: {
			id: id,
			solution: solution,
		},
		gas: '40000000000000',
	});
}

console.log('!!!!!!\tWelcome to NEAR-TreasureBoard by Armin FalDiS');
console.log('######\tPlease login using near-cli before using this tool');

let accountId = '';

while (true) {
	console.log(`
	Available actions are as followed:
	\t1. Start a new game
	\t2. Get the list of games
	\t3. Play a game
	\t4. Reveal the solution of a game
	\t0. Exit
	\n`);

	let action = Number(input.question('Pick your poison: ').trim());

	if (action == 0) {
		break;
	}

	// view methods don't need a signer
	if (action != 2) {
		accountId =
			input
				.question(
					`Enter your accountId${
						accountId.length ? ' (Leave blank to use ' + accountId + ')' : ''
					}: `,
				)
				.trim() || accountId;
		if (accountId.length == 0) {
			console.error(
				"\nYou've got to have an account ! Sorry, those are the rules friend",
			);
			continue;
		}
	}

	switch (action) {
		case 1:
			let size = input
				.question('Choose a size for this game (Small, Medium, Big): ')
				.trim();

			size = validate_size(size);
			let slots = get_slots(size);

			console.log('Now enter the number of slots that contain the bombs');
			let bombs = input
				.question(
					`(${slots / 2} numbers between 0-${slots - 1} seperated by SPACE): `,
				)
				.trim();

			let salt = input
				.question('Now enter a password to salt the solution with (UTF8): ')
				.trim();

			try {
				console.log(await newGame(accountId, size, slots, bombs, salt));
			} catch (err) {
				console.error(err.message);
			}
			break;
		case 2:
			let list;
			try {
				list = await games();
			} catch (err) {
				console.error(err.message);
				break;
			}
			if (list && list.length > 0) {
				let tbl = {};
				list.forEach((x) => {
					tbl[x.id] = {
						'Size': x.size,
						'Reserved slots':
							x.answers.length == get_slots(x.size) / 2 ? 'Closed' : x.answers,
					};
				});
				console.table(tbl);
			} else {
				console.log('\nThere are currently no games whatsoever!');
				console.log('*Pro tip: You could create a new one yourself :)');
			}
			break;
		case 3:
			let play_id = input.question('Enter the id of the game: ').trim();

			let choice = input.question('Which slot do you wish to choose? ').trim();

			if (!isNaN(play_id) && !isNaN(choice)) {
				try {
					console.log(await play(accountId, Number(play_id), Number(choice)));
				} catch (err) {
					console.error(err.message);
				}
			} else {
				console.error('\nThat was not a valid input !');
			}

			break;
		case 4:
			let reveal_id = input.question('Enter the id of the game: ').trim();

			let solution = input
				.question(
					'Now enter solution (The numbers provided during game creation): ',
				)
				.trim();

			
			let invalid_input = false;

			solution = solution.split(' ').map(x => {
				try {
					let s = Number(x);
					if(s < 0 || s > 255) {
						invalid_input = true;
					}
					return s;
				} catch {
					invalid_input = true;
					return -1;
				}
			});

			if(invalid_input) {
				console.error('The provided solution is invalid');
				continue;
			}

			if (!isNaN(reveal_id)) {
				try {
					console.log(await reveal(accountId, Number(reveal_id), solution));
				} catch (err) {
					console.error(err.message);
				}
			}

			break;
		default:
			console.log("\nSorry, I didn't quite catch that !");
	}
}
