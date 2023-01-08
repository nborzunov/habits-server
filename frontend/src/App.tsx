import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import Dashboard from '~/components/Dashboard/Dashboard';
import HabitsPage from '~/components/Habits/HabitsPage';
import Layout from '~/components/Layout/Layout';
import Auth from '~/components/Auth/Auth';
import Signup from '~/components/Auth/Signup';
import Login from '~/components/Auth/Login';
import AuthStartup from '~/components/Auth/AuthStartup';
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { User } from '~/types/types';
import api from '~/services/api';
import { useRecoilState, useSetRecoilState } from 'recoil';
import { activeUserState, tokenState } from '~/store/atoms';

function App() {
    const [isAuth, setIsAuth] = useState(true);

    const [token, setToken] = useRecoilState(tokenState);
    const setActiveUser = useSetRecoilState(activeUserState);

    const { refetch } = useQuery<User | null>({
        queryKey: ['active_user'],
        queryFn: async () => {
            if (!token) {
                setIsAuth(false);
                return Promise.resolve(null);
            }

            api.defaults.headers.common['Authorization'] = token as string;

            const user = await api
                .get<User>('/users/me')
                .then((res) => res.data)
                .catch(() => {
                    setToken(null);
                    setIsAuth(false);
                    return null;
                });

            setActiveUser(user);
            setIsAuth(true);
            return user;
        },
        initialData: null,
    });

    const handleRefetchUser = () => {
        refetch();
    };
    return (
        <BrowserRouter>
            <Routes>
                {!isAuth ? (
                    <Route path='/' element={<Auth />}>
                        <Route path='/' element={<AuthStartup />} />
                        <Route path='signup' element={<Signup refetch={handleRefetchUser} />} />
                        <Route path='login' element={<Login refetch={handleRefetchUser} />} />
                        <Route path='*' element={<Navigate to='/login' replace />} />
                    </Route>
                ) : (
                    <Route path='/' element={<Layout />}>
                        <Route index path='habits' element={<HabitsPage />} />
                        <Route path='dashboard' element={<Dashboard />} />
                        <Route path='*' element={<Navigate to='/habits' replace />} />
                    </Route>
                )}
            </Routes>
        </BrowserRouter>
    );
}

export default App;
