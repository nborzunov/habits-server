import { Box, Heading, List, Stack } from '@chakra-ui/react';
import HabitsListHeader from '~/Habits/components/HabitsList/HabitsListHeader';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import {
    activeUserState,
    completedHabitsState,
    habitsState,
    uncompletedHabitsState,
} from '~/common/store/atoms';
import { useQuery } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import { Habit } from '~/Habits/types';
import HabitItem from '~/Habits/components/HabitsList/HabitItem';

const HabitsList = () => {
    const activeUser = useRecoilValue(activeUserState);
    const setHabits = useSetRecoilState(habitsState);
    const uncompletedHabits = useRecoilValue(uncompletedHabitsState);
    const completedHabits = useRecoilValue(completedHabitsState);

    const { isLoading } = useQuery<Habit[]>({
        queryKey: ['habits'],
        queryFn: () =>
            api
                .get<Habit[]>('/habits/')
                .then((res) => res.data)
                .then((data) => {
                    setHabits(data);
                    return data;
                }),
        initialData: [],
        enabled: !!activeUser,
    });

    if (isLoading) return <div>Loading...</div>;

    const noHabits = uncompletedHabits.length === 0 && completedHabits.length === 0;

    return (
        <Box borderRightColor='gray.200' borderRightWidth='2px' h='100vh'>
            <HabitsListHeader />
            <Box>
                {noHabits && (
                    <Heading p={2} py={4} size={'md'} textAlign={'center'}>
                        No habits yet, let's create one!
                    </Heading>
                )}
                <Stack spacing={0}>
                    {uncompletedHabits.map((habit) => (
                        <HabitItem key={habit.id} habit={habit} />
                    ))}
                </Stack>
                {completedHabits.length > 0 && (
                    <Box mt={4}>
                        <Heading as='h3' size='md' mb={'12px'} py='8px' px={2}>
                            Completed today
                        </Heading>
                        <List styleType='none'>
                            {completedHabits.map((habit) => (
                                <HabitItem key={habit.id} habit={habit} />
                            ))}
                        </List>
                    </Box>
                )}
            </Box>
        </Box>
    );
};

export default HabitsList;
