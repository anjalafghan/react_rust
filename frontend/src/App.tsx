import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch("http://0.0.0.0:3000/display")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        return response.json();
      })
      .then((json) => setData(json))
      .catch((error) => console.log("Error: ", error));
  }, []);

  const json_data = JSON.parse(data);
  const keys = Object.keys(json_data.length ? json_data[0] : {});

  return (
    <div className="App">
      {json_data.length > 0 && (
        <table>
          <thead>
            <tr>
              {keys.map((item, idx) => (
                <th key={idx}>{item}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {json_data.map((item, idx) => (
              <tr key={idx}>
                {keys.map((key, idx) => (
                  <td>{item[key]}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

export default App;
