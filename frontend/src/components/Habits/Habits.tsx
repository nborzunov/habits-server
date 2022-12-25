import { Box, Grid, GridItem } from '@chakra-ui/react';
import { useEffect, useState } from 'react';
import { DataService } from '~/services/DataService';
import { Habit } from '~/types/types';
import HabitsList from '~/components/Habits/HabitsList';
import HabitDetails from '~/components/Habits/HabitDetails';

const Habits = () => {
    const [habits, setHabits] = useState<Habit[]>([]);

    const [selectedHabit, setSelectedHabit] = useState<Habit | null>(null);
    useEffect(() => {
        DataService.getHabits().then((res) => setHabits(res));
    }, []);

    return (
        <Box>
            <Grid templateColumns='350px 1fr'>
                <GridItem>
                    <HabitsList
                        habits={habits}
                        selectHabit={setSelectedHabit}
                        selectedHabit={selectedHabit}
                    />
                </GridItem>
                <GridItem p={2}>
                    <HabitDetails habit={selectedHabit} />
                </GridItem>
            </Grid>
        </Box>
    );
};

export default Habits;
