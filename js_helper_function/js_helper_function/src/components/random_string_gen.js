import bs58 from "bs58";

export const randomString = () => {
    const availableChars =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let randomString = "";
    for (let i = 0; i < 44; i++) {
      randomString +=
        availableChars[Math.floor(Math.random() * availableChars.length)];
    }
    return bs58.encode(Buffer.from(randomString)).toString().substring(0, 44);
  };