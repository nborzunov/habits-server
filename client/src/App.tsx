import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import Dashboard from '~/Dashboard/components/Dashboard';
import HabitsPage from '~/Habits/components/HabitsPage';
import Layout from '~/Layout/components/Layout';
import Auth from '~/Auth/components/Auth';
import Signup from '~/Auth/components/Signup';
import Login from '~/Auth/components/Login';
import AuthStartup from '~/Auth/components/AuthStartup';
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import { useRecoilState } from 'recoil';
import { activeUserState, tokenState } from '~/common/store/atoms';
import ProfilePage from '~/Profile/components/ProfilePage';
import { User } from '~/Profile/types';

function App() {
    const [isAuth, setIsAuth] = useState(true);

    const [token, setToken] = useRecoilState(tokenState);
    const [activeUser, setActiveUser] = useRecoilState(activeUserState);

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
                {!isAuth || !activeUser ? (
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
                        <Route path='me/*' element={<ProfilePage user={activeUser} />} />
                        <Route path='*' element={<Navigate to='/habits' replace />} />
                    </Route>
                )}
            </Routes>
        </BrowserRouter>
    );
}

export default App;
