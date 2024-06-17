import { useEffect, useState } from "react";
import "./App.css";
interface MonthlyDebit {
  month: string;
  total_debit: string;
  total_credit: string;
  balance: string;
}
function App() {
  const [data, setData] = useState<MonthlyDebit[]>([]);
  useEffect(() => {
    fetch("http://0.0.0.0:3000/get_monthly_debit")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        return response.json();
      })
      .then((json) => setData(json))
      .catch((error) => console.log("Error: ", error));
  }, []);

  return (
    <div className="container">
      <h2>Monthly Debit Table</h2>
      <table>
        <thead>
          <tr>
            <th>Month</th>
            <th>Total Debit</th>
            <th>Total Credit</th>
            <th>Balance</th>
          </tr>
        </thead>
        <tbody>
          {data.map((record, index) => (
            <tr key={index}>
              <td>{record.month}</td>
              <td>{record.total_debit}</td>
              <td>{record.total_credit}</td>
              <td>{record.balance}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
