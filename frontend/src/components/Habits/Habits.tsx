import { Box, Grid, GridItem } from '@chakra-ui/react';

import HabitsList from '~/components/Habits/HabitsList';
import HabitDetails from '~/components/Habits/HabitDetails';

const Habits = () => {
    return (
        <Box>
            <Grid templateColumns='350px 1fr'>
                <GridItem>
                    <HabitsList />
                </GridItem>
                <GridItem p={2}>
                    <HabitDetails />
                </GridItem>
            </Grid>
        </Box>
    );
};

export default Habits;
