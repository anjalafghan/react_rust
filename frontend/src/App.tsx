import "./App.css";
import { MonthlyAnalysis } from "./components/MonthlyAnalysis.tsx";
import { MyDatePicker } from "./components/DatePicker.tsx";
function App() {
  return (
    <div>
      <div>
        <MyDatePicker></MyDatePicker>
      </div>
      <div></div>
      <div>
        <MonthlyAnalysis></MonthlyAnalysis>
      </div>
    </div>
  );
}

export default App;
