const Buffer = require("buffer/").Buffer;
const borsh = require("borsh");

window.Buffer = Buffer;


export enum CurrencyType {
    Sol = 0,
    Spl
}

export enum InstructionData {
    Transfer = 0,
    CreatePayment
}


export class BorshSerializer {
    static encodeTxData(
        currency, seed, amount, 
        fee, status, shop_wallet, 
        hex_wallet
    ) {
        class Assignable {
            constructor(properties) {
                fee = Uint8Array.from([0,0,0,0,0,0,0,0]);
                Object.keys(properties).map((key) => {
                    if (key == "fee") return (this[key] = BorshSerializer.convertNumberToBinary(properties[key]));
                    return (this[key] = properties[key]);
                });
            }
        }
        /* 
        CreatePayment "Transaction Data" example

        Transfer:
          - Sol: [0, 0] (Transfer: 0, Sol: 0)
          - Spl: [0, 1] (Transfer: 0, Spl: 1)
        */
        class Payload extends Assignable {}
        const payloadSchema = new Map([[
            Payload,
            {
                kind: "struct",
                fields: [
                    ["instruction_data", "u8"],
                    ["seed", "string"],
                    ["amount", "u64"],
                    ["fee", [8]],
                    ["status", "string"],
                    ["shop_wallet", "string"],
                    ["hex_wallet", "string"]
                ],
            },
        ]]);
        let data = new Payload({
            instruction_data: currency,
            seed: seed,
            amount: amount,
            fee: fee,
            status: status,
            shop_wallet: shop_wallet,
            hex_wallet: hex_wallet
        });
        return Buffer.from(borsh.serialize(payloadSchema, data));
    }

    static convertNumberToBinary(num: number): Uint8Array {
        const c = new Uint8Array(new Float64Array([num]).buffer, 0, 8);
        return c;
    }
}