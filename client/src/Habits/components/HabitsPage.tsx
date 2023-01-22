import { Box, Grid, GridItem } from '@chakra-ui/react';

import HabitsList from '~/Habits/components/HabitsList/HabitsList';
import HabitDetails from '~/Habits/components/HabitDetails/HabitDetails';
import useTitle from '~/common/hooks/useTitle';

const HabitsPage = () => {
    useTitle('All Habits');
    return (
        <Box>
            <Grid templateColumns='460px 1fr'>
                <GridItem>
                    <HabitsList />
                </GridItem>
                <GridItem>
                    <HabitDetails />
                </GridItem>
            </Grid>
        </Box>
    );
};

export default HabitsPage;
