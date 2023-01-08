import { Box, Grid, GridItem, Heading } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import Statistics from '~/components/Habits/Statistics';
import { TargetCalendar, TargetCalendarContext } from '~/components/Dashboard/TargetCalendar';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import TargetChart from '~/components/Habits/TargetChart';
import { habitsState, selectedHabitState } from '~/store/atoms';
import { Habit, TargetType } from '~/types/types';
import { useMutation } from '@tanstack/react-query';
import api from '~/services/api';

const HabitDetails = () => {
    const habit = useRecoilValue(selectedHabitState);
    const setHabits = useSetRecoilState(habitsState);

    const createTarget = useMutation({
        mutationFn: (data: {
            id: string | null;
            habitId: string;
            date: Date;
            targetType: string;
        }) => {
            return api
                .post<Habit>('/targets/', data)
                .then((res) => res.data)
                .then((newHabit) =>
                    setHabits((prev) => prev.map((h) => (h.id !== newHabit.id ? h : newHabit))),
                );
        },
    });

    if (!habit) return null;

    const handleCalendarCellClick = (targetId: string | null, date: Date, newType: TargetType) => {
        createTarget.mutate({
            id: targetId,
            habitId: habit.id,
            date,
            targetType: newType,
        });
    };

    return (
        <Box>
            <Heading as='h3' px={2} size='md' mb={4}>
                {habit.title} header
            </Heading>
            <Box>
                <Grid gap={3} templateColumns='repeat(32, 1fr)'>
                    <GridItem colSpan={18}>
                        <Statistics
                            title='Current streak'
                            value={habit.currentStreak}
                            type='streak'
                            startDate={habit.currentStreakStartDate}
                        />
                    </GridItem>
                    {habit.completedTargets || habit.failedTargets ? (
                        <GridItem gridArea='1 / 19 / 5 / 26'>
                            <Box
                                borderRadius='xl'
                                borderColor='gray.200'
                                borderWidth='2px'
                                p='4'
                                height='390px'
                                display='flex'
                                justifyContent='center'
                            >
                                <TargetChart
                                    completed={habit.completedTargets}
                                    failed={habit.failedTargets}
                                />
                            </Box>
                        </GridItem>
                    ) : null}
                    <GridItem rowSpan={2} colStart={1} colSpan={9}>
                        <Statistics
                            icon={Icons.Complete}
                            title='Complete'
                            value={habit.completedTargets}
                            type='increase'
                            footerValue={habit.completedTargets}
                        />
                    </GridItem>
                    <GridItem rowSpan={2} colSpan={9}>
                        <Statistics
                            icon={Icons.Cross}
                            title='Failed'
                            value={habit.failedTargets}
                            type='increase'
                            footerValue={habit.failedTargets}
                        />
                    </GridItem>
                    <GridItem colStart={1} colSpan={18} display='flex' justifyContent='center'>
                        <Box
                            borderRadius='xl'
                            borderColor='gray.200'
                            borderWidth='2px'
                            p='2'
                            display='flex'
                            justifyContent='center'
                        >
                            <TargetCalendarContext.Provider
                                value={{
                                    habit,
                                    onCellClick: handleCalendarCellClick,
                                }}
                            >
                                <TargetCalendar targets={habit.targets} />
                            </TargetCalendarContext.Provider>
                        </Box>
                    </GridItem>
                </Grid>
            </Box>
        </Box>
    );
};

export default HabitDetails;
