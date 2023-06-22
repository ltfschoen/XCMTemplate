// Uses Mocha and Ganache
const Flipper = artifacts.require("Flipper");

contract('Flipper', accounts => {
    let flipper;
    const initValue = false;
    beforeEach(async () => {
        // Deploy token contract
        flipper = await Flipper.new(initValue, { from: accounts[0] });
    });
    // Check stored value
    it("checks stored value", async () => {
        const value = await flipper.get.call();
        assert.equal(value, initValue, 'value stored does not match initial value');
    });

    // Set flipped value of existing value
    it("should flip the value", async () => {
        const previousValue = await flipper.get.call();
        await flipper.flip.call({ from: accounts[0] });
        const newValue = await flipper.flip.call();
        assert.notEqual(previousValue, newValue, 'newValue is not opposite of previousValue');
    })
});
