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
    <InlineMath math="\displaystyle\int\limits_a^bx^2dx" />,
    <InlineMath math="\displaystyle\int\limits_a^b\sin(x)dx" />,
    <InlineMath math="\displaystyle\int\limits_a^be^xdx" />,
    <InlineMath math="\displaystyle\int\limits_a^b\dfrac{1}{x}dx" />,
    <InlineMath math="\displaystyle\int\limits_a^b\dfrac{1}{x^2-1}dx" />,
];

let methods = [
    <span>Left rectangles</span>,
    <span>Right rectangles</span>,
    <span>Middle rectangles</span>,
    <span>Trapezoids</span>,
    <span>Simpson's</span>,
];

let Fs = [
    function (x) {
        return x * x;
    },
    function (x) {
        return Math.sin(x);
    },
    function (x) {
        return Math.exp(x);
    },
    function (x) {
        return 1 / x;
    },
    function (x) {
        return 1 / (x * x - 1);
    }
]

function send(category, N, eps, lb, rb, method) {
    console.log(JSON.stringify({
        category: category,
        N: N,
        eps: eps,
        lb: lb,
        rb: rb,
        method: method
    }));
    return fetch("http://localhost:6379/", {
        method: "POST",
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        },
        body: JSON.stringify({
            category: category,
            N: N,
            eps: eps,
            lb: lb,
            rb: rb,
            method: method
        })
    })
        .then((response) => response.json());
}

function App() {
    let [category, setCategory] = useState(0);
    let [N, setN] = useState(4);
    let [eps, setEps] = useState(1.0);
    let [lb, setLB] = useState(0);
    let [rb, setRB] = useState(0);
    let [mb, setMB] = useState(0);

    let [result, setResult] = useState({});
    let [checked, setChecked] = useState(false);

    let init = <div>
        <span>Choose integral:</span>
        {equations.map((equation, index) => (
            <div key={index}>
                <div>
                    <Checkbox style={{ margin: "-105px 15px 25px -15px" }} inputId={"checkbox" + index} value={""} id={"checkbox" + index} name={"checkbox"} checked={index === category} onChange={() => setCategory(index)} />
                    <label htmlFor={"checkbox" + index}>{equation}</label>
                </div>
                <br />
            </div>
        ))}
        <br />
        <span>Choose <InlineMath math="n \geq 4" />:</span><br />
        <InputNumber value={N} style={{ width: "10px", marginTop: "10px", marginLeft: "-240px" }} useGrouping={false} min={4} onValueChange={(e) => setN(e.value)} />
        <br /><br />
        <span>Choose <InlineMath math="\varepsilon" />:</span><br />
        <InputNumber value={eps} style={{ width: "10px", marginTop: "10px", marginLeft: "-240px" }} useGrouping={false} min={0.00001} maxFractionDigits={5} onValueChange={(e) => setEps(e.value)} />
        <br />
        {
            <div style={{ marginLeft: "-220px" }}>
                <span>Left bound: </span><InputNumber value={lb} style={{ width: "10px", marginTop: "10px" }} useGrouping={false} onValueChange={(e) => setLB(e.value)} maxFractionDigits={5} />
                <br />
                <span>Right bound: </span><InputNumber value={rb} style={{ width: "10px", marginTop: "10px" }} useGrouping={false} onValueChange={(e) => setRB(e.value)} maxFractionDigits={5} />
                <br />
                <span style={{ marginLeft: "120px" }}>Method: </span><Dropdown style={{ marginLeft: "10px" }} useGrouping={false} value={methods[mb]} options={methods} onChange={(e) => setMB(methods.indexOf(e.value))} />
                <br />
            </div>
        }
        <br />
    </div>;

    return (
        <div className="" style={{ textAlign: "center", marginTop: "100px" }}>
            <div id="basics">
                {!checked && init}
                {!checked && <Button label="Submit" onClick={() => {
                    let res = send(category, N, eps, lb, rb, mb);
                    res.then(d => {
                        setResult(d);
                        setChecked(true);
                        console.log(d);

                        var riemannsum_vars = ['left', 'right', 'middle', 'trapezoidal', 'simpson'];
                        console.log(riemannsum_vars[mb]);

                        board.create('functiongraph', [Fs[category], function () { return lb; }, function () { return rb; }]);
                        var a = board.create('riemannsum', [
                            Fs[category],
                            parseInt(d.N), function () { return riemannsum_vars[mb]; },
                            lb,
                            rb,
                        ],
                            { fillColor: '#ADADAD', fillOpacity: 0.3 });
                        console.log(a);
                        document.getElementById("board").style.display = 'block';
                    });
                }} />}
                {checked && <div id="result" style={{ textAlign: "center" }}>
                    <span>Result: <InlineMath math={parseInt(result.x).toString()} /></span>
                    <br />
                    <span>N: <InlineMath math={result.N.toString()} /></span>
                    <br />
                    <span>Loss: <InlineMath math={parseInt(result.acc).toString()} /></span>
                    <br />
                    <span>Errors: {result.errors.toString()} </span>
                    <br /><br />
                </div>}
            </div>
        </div >
    );
}

export default App;
