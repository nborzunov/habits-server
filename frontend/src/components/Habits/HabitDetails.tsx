import { Box, Grid, GridItem, Heading } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import { Habit } from '~/types/types';
import Statistics from '~/components/Habits/Statistics';
import TargetCalendar from '~/components/Dashboard/TargetCalendar';

import TargetChart from '~/components/Habits/TargetChart';

const HabitDetails = ({ habit }: { habit: Habit | null }) => {
    if (!habit) return null;
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
                    <GridItem gridArea='1 / 19 / 5 / 27'>
                        <Box
                            borderRadius='xl'
                            borderColor='gray.200'
                            borderWidth='2px'
                            p='4'
                            height='390px'
                        >
                            <TargetChart />
                        </Box>
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
                    <GridItem colStart={1} colSpan={18} display='flex' justifyContent='center'>
                        <Box
                            borderRadius='xl'
                            borderColor='gray.200'
                            borderWidth='2px'
                            p='2'
                            display='flex'
                            width='100%'
                            justifyContent='center'
                        >
                            <TargetCalendar targets={habit.targets} />
                        </Box>
                    </GridItem>
                </Grid>
            </Box>
        </Box>
    );
};

export default HabitDetails;
