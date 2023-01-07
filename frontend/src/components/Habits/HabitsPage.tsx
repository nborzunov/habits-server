import { Box, Grid, GridItem } from '@chakra-ui/react';

import HabitsList from '~/components/Habits/HabitsList';
import HabitDetails from '~/components/Habits/HabitDetails';

const HabitsPage = () => {
    return (
        <Box>
            <Grid templateColumns='460px 1fr'>
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

export default HabitsPage;
