const getWeb3 = () => {
  return new Promise((resolve, reject) => {
    window.addEventListener("load", async () => {
      if (window.ethereum) {
        const web3 = new Web3(window.ethereum);
        try {
          // ask user permission to access his accounts
          await window.ethereum.request({ method: "eth_requestAccounts" });
          resolve(web3);
        } catch (error) {
          reject(error);
        }
      } else {
        reject("must install MetaMask");
      }
    });
  });
};

const getContract = async (web3) => {
	const data = await $.getJSON('/contracts/flipper.sol/build/contracts/Flipper.json');
	const contract = new web3.eth.Contract(data.abi, "0xcCF89DAfeF6634fd058F356F8f2650eae2c93Bef");
	return contract;
};

// Wrap the code inside an async function
async function MyFn() {
	 const web3 = await getWeb3();
	 const accounts = await web3.eth.getAccounts();
	 const contract = await getContract(web3);

	// --- Sending Tokens
	// Add event listener to the Send Transaction button
	$('#sendButton').on('click', async () => {
					try {
						await web3.eth.sendTransaction({
							from: accounts[0],
							to:"0x7BF369283338E12C90514468aa3868A551AB2929",
							value: 1000000000000000000,
							gasLimit: 21000
						})
					} catch (error) {
						console.error(error);
					}
	});

	// --- Calling Contract
	// Add event listener to the Send Transaction button
	$('#callButton').on('click', async () => {
			try {
				  // call contract method and send
				  await contract.methods
					.flip()
					  .send({
						  from: accounts[0],
						  gas: '100000000',
						  // other transaction's params
					  })
				} catch (error) {
						console.error(error);
				}
			});
}

MyFn();
