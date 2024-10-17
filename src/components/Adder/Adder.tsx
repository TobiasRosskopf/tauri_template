import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function Adder() {
    // State for two numbers and their sum
    const [num1, setNum1] = useState(0);
    const [num2, setNum2] = useState(0);
    const [sum, setSum] = useState(0);

    // Call and wait for rust command to add two numbers
    async function add() {
        setSum(await invoke("add", { num1, num2 }));
    }

    // Render the form and the result
    return (
        <div>
            <div className="row">
                <form
                onSubmit={(e) => {
                    e.preventDefault();
                    add();
                }}
                >
                <input
                    id="num1-input"
                    onChange={(e) => setNum1(parseInt(e.currentTarget.value))}
                    placeholder="Number 1"
                />
                <input
                    id="num2-input"
                    onChange={(e) => setNum2(parseInt(e.currentTarget.value))}
                    placeholder="Number 2"
                />
                <button type="submit">Add</button>
                </form>
            </div>
            <p>{sum}</p>
        </div>
    );
}

export default Adder;
