'use strict';

const KEY = 'foobar';
const PLAINTEXT = 'abcdefghijklmnopqrstuvwxyz';

const arc4 = require('arc4');

const cipher = arc4('arc4', KEY);

const message = cipher.encodeString(PLAINTEXT, 'ascii', 'hex');
console.log(message);


