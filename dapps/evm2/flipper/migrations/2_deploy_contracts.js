var Flipper = artifacts.require("./Flipper");

module.exports = function (deployer) {
  deployer.deploy(Flipper, false);
};
