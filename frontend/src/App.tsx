import { BrowserRouter, Route, Routes } from 'react-router-dom';
import Dashboard from '~/components/Dashboard/Dashboard';
import Habits from '~/components/Habits/Habits';
import Layout from '~/components/Layout/Layout';

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route index element={<div></div>} />
                <Route path='/' element={<Layout />}>
                    <Route path='habits' element={<Habits />} />
                    <Route path='dashboard' element={<Dashboard />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;
