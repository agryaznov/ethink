const MyContract = artifacts.require("Flipper");
module.exports = function(deployer) {
    deployer.deploy(MyContract, false);
};
