const chai = require('chai');
const chaiHttp = require('chai-http');
const app = require('../index'); // Assuming your Express.js application file is named index.js
const expect = chai.expect;

// Configure chai to use chai-http plugin
chai.use(chaiHttp);

describe('Blockchain Bridge API', () => {
  // Test the root route ("/")
  describe('GET /', () => {
    it('should return status 200 and a welcome message', async () => {
      const res = await chai.request(app).get('/');
      expect(res).to.have.status(200);
      expect(res.text).to.equal('Welcome to the Blockchain Bridge API');
    });
  });

  // Test the '/lock' route
  describe('POST /lock', () => {
    it('should return status 200 and a success message', async () => {
      const tokenData = { token: 'ETH', amount: 10 };
      const res = await chai.request(app).post('/lock').send(tokenData);
      expect(res).to.have.status(200);
      expect(res.body).to.be.an('object');
      expect(res.body).to.have.property('status').to.equal('success');
      expect(res.body).to.have.property('message').to.equal('Token locked successfully');
    });
  });

  // Test the '/release' route
  describe('POST /release', () => {
    it('should return status 200 and a success message', async () => {
      const lockEventData = { token: 'ETH', amount: 10, targetAddress: '0x123456789' };
      const res = await chai.request(app).post('/release').send(lockEventData);
      expect(res).to.have.status(200);
      expect(res.body).to.be.an('object');
      expect(res.body).to.have.property('status').to.equal('success');
      expect(res.body).to.have.property('message').to.equal('Token released successfully');
    });
  });
});
