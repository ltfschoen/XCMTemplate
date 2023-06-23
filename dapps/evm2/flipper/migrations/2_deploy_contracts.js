var RandomNumber = artifacts.require("./lib/RandomNumber");
var Flipper = artifacts.require("./Flipper");

module.exports = function (deployer) {
  // deployer.deploy(RandomnessConsumer);
  deployer.deploy(RandomNumber);
  deployer.deploy(Flipper, false);
};
