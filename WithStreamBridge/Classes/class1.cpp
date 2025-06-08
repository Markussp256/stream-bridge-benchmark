#include "class1.h"
#include "ostream-bridge.h"

void Class1 :: print(stream::ostream& os) const
{
	os << "this is Class1";
};