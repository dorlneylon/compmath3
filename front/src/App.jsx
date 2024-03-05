import { useState, React } from "react";
import "./App.css";
import 'katex/dist/katex.min.css'
import { InlineMath } from 'react-katex';
import { Checkbox } from 'primereact/checkbox';
import { InputNumber } from 'primereact/inputnumber';
import { Button } from 'primereact/button';
import { Dropdown } from 'primereact/dropdown';
import "primereact/resources/themes/bootstrap4-dark-purple/theme.css";

let equations = [
    <InlineMath math="x^3 - x + 4 = 0" />,
    <InlineMath math="\sin(x) = 0" />,
    <InlineMath math="\dfrac{12}{11}x - \dfrac{1}{11}x^3 - \dfrac{4}{11} = 0" />
];

let systems = [
    <InlineMath math="\left\{ \begin{array}{l} x - (0.3 - 0.1x^2 - 0.2y^2) = 0, \\ y - (0.7 - 0.2x^2 - 0.1xy) = 0 \end{array} \right." />,
    <InlineMath math="\left\{ \begin{array}{l} \sin\left(y+2\right)-x=1.5, \\ y+\cos\left(x-2\right)=0.5 \end{array} \right." />,
];

let methods = [
    <span>Chords</span>,
    <span>Secants</span>,
    <span>SimpleIt</span>,
];

function send(category, eps, lb, rb, start, method, board) {
    if (category < 3) {
        let bd = JSON.stringify({
            category,
            eps,
            lb,
            rb,
            method
        });
        console.log(bd);
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            },
            body: bd,
        };

        let res = fetch('http://127.0.0.1:6379/', requestOptions)
            .then(response => response.json())
            .catch(error => console.error('Error:', error));

        return res;
    } else {
        let bd = JSON.stringify({
            category,
            eps,
            lb: start[0],
            rb: start[1],
            method
        });
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            },
            body: bd,
        };

        console.log(bd);

        let res = fetch('http://127.0.0.1:6379/', requestOptions)
            .then(response => response.json())
            .catch(error => console.error('Error:', error));

        return res;
    }
}

function getFun(category) {
    if (category == 0) {
        return (x) => x ** 3 - x + 4;
    } else if (category == 1) {
        return (x) => Math.sin(x);
    } else {
        return (x) => 12.0 / 11.0 * x - 1.0 / 11.0 * x ** 3 - 4.0 / 11.0;
    }
}

function getSys(category) {
    if (category == 3) {
        return [(x, y) => x - (0.3 - 0.1 * x ** 2 - 0.2 * y ** 2), (x, y) => y - (0.7 - 0.2 * x ** 2 - 0.1 * x * y)];
    } else {
        return [(x, y) => Math.sin(y + 2) - x - 1.5, (x, y) => y + Math.cos(x - 2) - 0.5];
    }
}

function App() {
    let [category, setCategory] = useState(0);
    let [eps, setEps] = useState(0.0001);
    let [lb, setLB] = useState(0);
    let [rb, setRB] = useState(0);
    let [start, setStart] = useState([0, 0]);
    let [method, setMethod] = useState(0);
    let [mb, setMb] = useState(<span></span>);

    let [result, setResult] = useState({});
    let [checked, setChecked] = useState(false);

    let init = <div>
        <span>Choose equation or system:</span>
        {equations.map((equation, index) => (
            <div key={index}>
                <Checkbox style={{ margin: "5px 15px 5px -15px" }} inputId={"checkbox" + index} value={""} id={"checkbox" + index} name={"checkbox"} checked={index === category} onChange={() => setCategory(index)} />
                <label htmlFor={"checkbox" + index}>{equation}</label>
                <br />
            </div>
        ))}
        {systems.map((system, index) => (
            <div key={index} style={{ marginLeft: "100px", marginTop: "10px" }}>
                <Checkbox style={{ margin: "5px 15px 20px -105px" }} inputId={"checkboxS" + index} value={""} id={"checkboxS" + index} name={"checkboxS"} checked={index + 3 === category} onChange={() => setCategory(index + 3)} />
                <label htmlFor={"checkbox" + index}>{system}</label>
                <br />
            </div>
        ))}
        <br />
        <span>Choose <InlineMath math="\varepsilon" />:</span><br />
        <InputNumber value={eps} style={{ width: "10px", marginTop: "10px", marginLeft: "-240px" }} onValueChange={(e) => setEps(e.value)} minFractionDigits={2} maxFractionDigits={5} />
        <br />
        {category < 3 &&
            <div style={{ marginLeft: "-220px" }}>
                <span>Left bound: </span><InputNumber value={lb} style={{ width: "10px", marginTop: "10px" }} onValueChange={(e) => setLB(e.value)} maxFractionDigits={5} />
                <br />
                <span>Right bound: </span><InputNumber value={rb} style={{ width: "10px", marginTop: "10px" }} onValueChange={(e) => setRB(e.value)} maxFractionDigits={5} />
                <br />
                <span style={{ marginLeft: "120px" }}>Method: </span><Dropdown style={{ marginLeft: "10px" }} value={mb} options={methods} onChange={(e) => { setMb(e.value); setMethod(methods.indexOf(e.value)); }} />
                <br />
            </div>
        }
        {category > 2 &&
            <div style={{ marginLeft: "-220px" }}>
                <span>Choose init value <InlineMath math="x" />: </span><InputNumber value={start[0]} style={{ width: "10px", marginTop: "10px" }} onValueChange={(e) => setStart([e.value, start[1]])} maxFractionDigits={5} />
                <br />
                <span>Choose init value <InlineMath math="y" />: </span><InputNumber value={start[1]} style={{ width: "10px", marginTop: "10px" }} onValueChange={(e) => setStart([start[0], e.value])} maxFractionDigits={5} />
                <br />
            </div>
        }
        <input type="file" id="fileInput" style={{ marginLeft: "-10px", marginTop: "20px" }} onChange={(e) => {
            const file = e.target.files[0];
            let reader = new FileReader();
            reader.onload = function (e) {
                const data = JSON.parse(e.target.result);
                setCategory(data.category);
                setEps(data.eps);
                setLB(data.lb);
                setRB(data.rb);
                setStart(data.start);
                setMethod(data.method);
            }
            reader.readAsText(file);
        }} />
        <br />
        <br />
    </div>;

    return (
        <div className="" style={{ textAlign: "center", marginTop: "100px" }}>
            <div id="basics">
                {!checked && init}
                {!checked && <Button label="Submit" onClick={() => {
                    console.log(category, eps, lb, rb, start, method);
                    let res = send(category, eps, lb, rb, start, method, board);
                    res.then(d => {
                        setResult(d);
                        setChecked(true);
                        console.log(d);
                        if (category < 3) {
                            board.create('functiongraph', [getFun(category)], lb, rb, { strokeWidth: 2, strokeColor: 'blue' });
                            board.create('point', [d.x[0], d.x[1]], { fillColor: 'red' });
                        } else {
                            board.create('implicitcurve', [getSys(category)[0]], {
                                strokeWidth: 3,
                                strokeOpacity: 0.8,
                                resolution_outer: 20,
                                resolution_inner: 20
                            });
                            board.create('implicitcurve', [getSys(category)[1]], {
                                strokeWidth: 3,
                                strokeOpacity: 0.8,
                                strokeColor: 'green',
                                resolution_outer: 20,
                                resolution_inner: 20
                            });
                            board.create('point', [d.x[0], d.x[1]], { fillColor: 'red' });
                        }
                        document.getElementById("board").style.display = 'block';
                    });
                }} />}
                {checked && <div id="result" style={{ textAlign: "center" }}>
                    <span>Result: <InlineMath math={"(" + result.x[0] + ", " + result.x[1] + ")"} /></span>
                    <br />
                    <span>Loss: <InlineMath math={category < 3 ? result.acc.toString() : "(" + result.acc[0] + ", " + result.acc[1] + ")"} /></span>
                    <br />
                    <span>Iterations: <InlineMath math={result.iters.toString()} /></span>
                    <br />
                    <span>Errors: {result.errors.toString()} </span>
                    <br /><br /><br /><br /><br /><br />
                </div>}
            </div>
        </div >
    );
}

export default App;
