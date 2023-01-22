import { Icon, IconButton, Tooltip } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';
import { Habit, TargetType } from '~/Habits/types';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import { habitsState } from '~/common/store/atoms';
import { useSetRecoilState } from 'recoil';
import dayjs from 'dayjs';
import getCorrentDate from '~/common/utils/getCurrectDate';

const CompletedCheckbox = ({ value, habit }: { value: boolean; habit: Habit }) => {
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

    const handleChange = (e: any) => {
        e.preventDefault();
        e.stopPropagation();
        createTarget.mutate({
            id:
                habit.targets.find(
                    (t) => dayjs(t.date).format('DD MM YYYY') === dayjs().format('DD MM YYYY'),
                )?.id || null,
            date: getCorrentDate(dayjs().toDate()),
            habitId: habit.id,
            targetType: value ? TargetType.Empty : TargetType.Done,
        });
    };

    return (
        <Tooltip label={value ? 'Uncheck' : 'Complete'}>
            <IconButton
                borderRadius={'full'}
                borderWidth={'2px'}
                variant={value ? 'solid' : 'outline'}
                colorScheme={'purple'}
                size={'sm'}
                icon={<Icon as={Icons.Complete} />}
                aria-label={'complete'}
                mr={'2'}
                onClick={handleChange}
            />
        </Tooltip>
    );
};

export default CompletedCheckbox;
