//This Application contains all routes for NIORA

import {Routes, Route} from 'react-router-dom';


//Application Endpoints
import Home from '../screens/Home';

const AppRoutes = () => {
    return (
        <main>
            <Routes>
                <Route path = "/" element={<Home/>}/>
            </Routes>
        </main>
    );
};

export default AppRoutes;