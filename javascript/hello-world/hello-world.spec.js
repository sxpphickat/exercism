import { hello, HELLOAAA } from './hello-world';
describe('Hello World', () => {
  test('Say Hi!', () => {
    expect(hello()).toEqual('Hello, World!');
  });
});

console.log(HELLOAAA);