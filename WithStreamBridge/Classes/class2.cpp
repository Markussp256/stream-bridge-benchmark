#include "class2.h"
#include "ostream-bridge.h"

void Class2 :: print(stream::ostream& os) const
{
	os << "this is Class2";
};