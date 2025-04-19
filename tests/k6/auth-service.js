import http from 'k6/http';
import {check, sleep} from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';


export let options = {
    vus: 100,  // 10 Virtual Users
    duration: '60s',  // Run test for 30 seconds
};

    let host = 'http://192.168.1.103:3000'; // Adjust for your API host

export default function () {

    let url = `${host}/register`; // Adjust for your API

    const randomFirstName = randomString(8);

    let payload = JSON.stringify({
        email: `${randomFirstName}@mail.com`,
        password: 'password123',
    });

    let params = {
        headers: {'Content-Type': 'application/json'},
    };

    let res = http.post(url, payload, params);

    check(res, {
        'is status 200': (r) => r.status === 200,
        'response time < 500ms': (r) => r.timings.duration < 500,
    });

    // sleep(1);
}