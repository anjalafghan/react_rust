import { useEffect, useState } from "react";
import {CartesianGrid, Legend, Line, LineChart, Tooltip, XAxis, YAxis } from "recharts";

interface MonthlyDebit {
  month: string;
  total_debit: string;
  total_credit: string;
  balance: string;
}
export function MonthlyAnalysis() {
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
        <LineChart width={730} height={250} data={data}
                   margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
            <CartesianGrid />
            <XAxis dataKey="" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Line name="monthly average balance" type="monotone" dataKey="balance" stroke="#8884d8" strokeWidth={4}/>
        </LineChart>
  );
}
