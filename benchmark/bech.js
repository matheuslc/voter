import grpc from 'k6/net/grpc';
import { check, sleep } from 'k6';

const client = new grpc.Client();
client.load(['definitions'], '../../proto/voter.proto');

export default () => {
    client.connect('localhost:50051', { plaintext: true, reflect: false });
  
    const data = {
        "option_id": "1",
        "poll_id": "1",
        "user_id": "1"
    };

    const response = client.invoke('voterproto.VoteService/Vote', data);
  
    check(response, {
      'status is OK': (r) => r && r.status === grpc.StatusOK,
    });
  
    console.log(JSON.stringify(response.message));
  
    client.close();
  };

  export const options = {
    vus: 100,
    duration: '1m',
  };