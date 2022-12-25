import { Box, Flex, Grid, GridItem, Heading } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import { Habit } from '~/types/types';
import Statistics from '~/components/Habits/Statistics';
import TargetCalendar from '../Dashboard/TargetCalendar';

const HabitDetails = ({ habit }: { habit: Habit | null }) => {
    if (!habit) return null;
    return (
        <Box>
            <Heading as='h3' px={2} size='md' mb={4}>
                {habit.title} header
            </Heading>
            <Box>
                {/* <p>Periodicity: {habit.periodicity}</p>
                {habit.periodicityValue && <p>Periodicity Value: {habit.periodicityValue}</p>}
                <p>Activity Type: {habit.activityType}</p>
                {habit.activityCounterValue !== undefined && (
                    <p>Activity Counter Value: {habit.activityCounterValue}</p>
                )}
                <p>
                    Goal: {habit.goal} {habit.goalType}
                </p>
                <p>Create Date: {habit.createDate.toDateString()}</p>
                <p>Start Date: {habit.startDate.toDateString()}</p>
                <p>Current Streak: {habit.currentStreak}</p>
                <p>Current Streak Start Date: {habit.currentStreakStartDate.toDateString()}</p>
                <p>Completed Targets: {habit.completedTargets}</p>
                <p>Failed Targets: {habit.failedTargets}</p>
                <p>Total Targets: {habit.totalTargets}</p>
                {habit.targets.length > 0 && (
                    <>
                        <h2>Targets</h2>
                        <ul>
                            {habit.targets.map((target) => (
                                <li key={target.id}>
                                    {target.date.toDateString()}: {target.type}
                                </li>
                            ))}
                        </ul>
                    </>
                )} */}
                <Grid gap={3} templateColumns='repeat(32, 1fr)'>
                    <GridItem colSpan={18}>
                        <Statistics
                            title='Current streak'
                            value={habit.currentStreak}
                            type='streak'
                            startDate={habit.currentStreakStartDate}
                        />
                    </GridItem>
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
                    <GridItem colStart={1} colSpan={18} as={Flex}>
                        <Box borderRadius='xl' borderColor='gray.200' borderWidth='2px' p='2'>
                            <TargetCalendar size='sm' />
                        </Box>
                    </GridItem>
                </Grid>
            </Box>
        </Box>
    );
};

export default HabitDetails;
