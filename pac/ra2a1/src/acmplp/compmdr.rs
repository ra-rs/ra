#[doc = "Register `COMPMDR` reader"]
pub struct R(crate::R<COMPMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPMDR` writer"]
pub struct W(crate::W<COMPMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<COMPMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0ENB` reader - ACMPLP0 Operation Enable"]
pub type C0ENB_R = crate::BitReader<C0ENB_A>;
#[doc = "ACMPLP0 Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ENB_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0ENB_A> for bool {
    #[inline(always)]
    fn from(variant: C0ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl C0ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0ENB_A {
        match self.bits {
            false => C0ENB_A::_0,
            true => C0ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0ENB_A::_1
    }
}
#[doc = "Field `C0ENB` writer - ACMPLP0 Operation Enable"]
pub type C0ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C0ENB_A, O>;
impl<'a, const O: u8> C0ENB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0ENB_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0ENB_A::_1)
    }
}
#[doc = "Field `C0WDE` reader - ACMPLP0 Window Function Mode Enable"]
pub type C0WDE_R = crate::BitReader<C0WDE_A>;
#[doc = "ACMPLP0 Window Function Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0WDE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0WDE_A> for bool {
    #[inline(always)]
    fn from(variant: C0WDE_A) -> Self {
        variant as u8 != 0
    }
}
impl C0WDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0WDE_A {
        match self.bits {
            false => C0WDE_A::_0,
            true => C0WDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0WDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0WDE_A::_1
    }
}
#[doc = "Field `C0WDE` writer - ACMPLP0 Window Function Mode Enable"]
pub type C0WDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C0WDE_A, O>;
impl<'a, const O: u8> C0WDE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0WDE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0WDE_A::_1)
    }
}
#[doc = "Field `C0VRF` reader - ACMPLP0 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
pub type C0VRF_R = crate::BitReader<C0VRF_A>;
#[doc = "ACMPLP0 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0VRF_A {
    #[doc = "0: CMPREF0 input"]
    _0 = 0,
    #[doc = "1: internal reference voltage (Vref)"]
    _1 = 1,
}
impl From<C0VRF_A> for bool {
    #[inline(always)]
    fn from(variant: C0VRF_A) -> Self {
        variant as u8 != 0
    }
}
impl C0VRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0VRF_A {
        match self.bits {
            false => C0VRF_A::_0,
            true => C0VRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0VRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0VRF_A::_1
    }
}
#[doc = "Field `C0VRF` writer - ACMPLP0 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
pub type C0VRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C0VRF_A, O>;
impl<'a, const O: u8> C0VRF_W<'a, O> {
    #[doc = "CMPREF0 input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0VRF_A::_0)
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0VRF_A::_1)
    }
}
#[doc = "Field `C0MON` reader - ACMPLP0 Monitor Flag"]
pub type C0MON_R = crate::BitReader<C0MON_A>;
#[doc = "ACMPLP0 Monitor Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0MON_A {
    #[doc = "0: IVCMP0 < Comparator0 Reference level(When the window function is disabled)/IVCMP0 < IVREF0 or IVCMP0 > IVREF1(When the window function is enabled)"]
    _0 = 0,
    #[doc = "1: IVCMP0 > Comparator0 Reference level(When the window function is disabled)/IVREF0 < IVCMP0 < IVREF1(When the window function is enabled)"]
    _1 = 1,
}
impl From<C0MON_A> for bool {
    #[inline(always)]
    fn from(variant: C0MON_A) -> Self {
        variant as u8 != 0
    }
}
impl C0MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0MON_A {
        match self.bits {
            false => C0MON_A::_0,
            true => C0MON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0MON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0MON_A::_1
    }
}
#[doc = "Field `C0MON` writer - ACMPLP0 Monitor Flag"]
pub type C0MON_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C0MON_A, O>;
impl<'a, const O: u8> C0MON_W<'a, O> {
    #[doc = "IVCMP0 < Comparator0 Reference level(When the window function is disabled)/IVCMP0 < IVREF0 or IVCMP0 > IVREF1(When the window function is enabled)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C0MON_A::_0)
    }
    #[doc = "IVCMP0 > Comparator0 Reference level(When the window function is disabled)/IVREF0 < IVCMP0 < IVREF1(When the window function is enabled)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C0MON_A::_1)
    }
}
#[doc = "Field `C1ENB` reader - ACMPLP1 Operation Enable"]
pub type C1ENB_R = crate::BitReader<C1ENB_A>;
#[doc = "ACMPLP1 Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ENB_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1ENB_A> for bool {
    #[inline(always)]
    fn from(variant: C1ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl C1ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1ENB_A {
        match self.bits {
            false => C1ENB_A::_0,
            true => C1ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1ENB_A::_1
    }
}
#[doc = "Field `C1ENB` writer - ACMPLP1 Operation Enable"]
pub type C1ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C1ENB_A, O>;
impl<'a, const O: u8> C1ENB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1ENB_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1ENB_A::_1)
    }
}
#[doc = "Field `C1WDE` reader - ACMPLP1 Window Function Mode Enable"]
pub type C1WDE_R = crate::BitReader<C1WDE_A>;
#[doc = "ACMPLP1 Window Function Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1WDE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1WDE_A> for bool {
    #[inline(always)]
    fn from(variant: C1WDE_A) -> Self {
        variant as u8 != 0
    }
}
impl C1WDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1WDE_A {
        match self.bits {
            false => C1WDE_A::_0,
            true => C1WDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1WDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1WDE_A::_1
    }
}
#[doc = "Field `C1WDE` writer - ACMPLP1 Window Function Mode Enable"]
pub type C1WDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C1WDE_A, O>;
impl<'a, const O: u8> C1WDE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1WDE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1WDE_A::_1)
    }
}
#[doc = "Field `C1VRF` reader - ACMPLP1 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
pub type C1VRF_R = crate::BitReader<C1VRF_A>;
#[doc = "ACMPLP1 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1VRF_A {
    #[doc = "0: CMPREF1 input"]
    _0 = 0,
    #[doc = "1: internal reference voltage (Vref)"]
    _1 = 1,
}
impl From<C1VRF_A> for bool {
    #[inline(always)]
    fn from(variant: C1VRF_A) -> Self {
        variant as u8 != 0
    }
}
impl C1VRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1VRF_A {
        match self.bits {
            false => C1VRF_A::_0,
            true => C1VRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1VRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1VRF_A::_1
    }
}
#[doc = "Field `C1VRF` writer - ACMPLP1 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
pub type C1VRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C1VRF_A, O>;
impl<'a, const O: u8> C1VRF_W<'a, O> {
    #[doc = "CMPREF1 input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1VRF_A::_0)
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1VRF_A::_1)
    }
}
#[doc = "Field `C1MON` reader - ACMPLP1 Monitor Flag"]
pub type C1MON_R = crate::BitReader<C1MON_A>;
#[doc = "ACMPLP1 Monitor Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1MON_A {
    #[doc = "0: IVCMP1 < Comparator1 Reference level(When the window function is disabled)/IVCMP1 < IVREF0 or IVCMP1 > IVREF1(When the window function is enabled)"]
    _0 = 0,
    #[doc = "1: IVCMP1 > Comparator1 Reference level(When the window function is disabled)/IVREF0 < IVCMP1 < IVREF1(When the window function is enabled)"]
    _1 = 1,
}
impl From<C1MON_A> for bool {
    #[inline(always)]
    fn from(variant: C1MON_A) -> Self {
        variant as u8 != 0
    }
}
impl C1MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1MON_A {
        match self.bits {
            false => C1MON_A::_0,
            true => C1MON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1MON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1MON_A::_1
    }
}
#[doc = "Field `C1MON` writer - ACMPLP1 Monitor Flag"]
pub type C1MON_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMPMDR_SPEC, C1MON_A, O>;
impl<'a, const O: u8> C1MON_W<'a, O> {
    #[doc = "IVCMP1 < Comparator1 Reference level(When the window function is disabled)/IVCMP1 < IVREF0 or IVCMP1 > IVREF1(When the window function is enabled)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(C1MON_A::_0)
    }
    #[doc = "IVCMP1 > Comparator1 Reference level(When the window function is disabled)/IVREF0 < IVCMP1 < IVREF1(When the window function is enabled)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(C1MON_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ACMPLP0 Operation Enable"]
    #[inline(always)]
    pub fn c0enb(&self) -> C0ENB_R {
        C0ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACMPLP0 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c0wde(&self) -> C0WDE_R {
        C0WDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMPLP0 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
    #[inline(always)]
    pub fn c0vrf(&self) -> C0VRF_R {
        C0VRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACMPLP0 Monitor Flag"]
    #[inline(always)]
    pub fn c0mon(&self) -> C0MON_R {
        C0MON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACMPLP1 Operation Enable"]
    #[inline(always)]
    pub fn c1enb(&self) -> C1ENB_R {
        C1ENB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMPLP1 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c1wde(&self) -> C1WDE_R {
        C1WDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACMPLP1 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
    #[inline(always)]
    pub fn c1vrf(&self) -> C1VRF_R {
        C1VRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP1 Monitor Flag"]
    #[inline(always)]
    pub fn c1mon(&self) -> C1MON_R {
        C1MON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMPLP0 Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c0enb(&mut self) -> C0ENB_W<0> {
        C0ENB_W::new(self)
    }
    #[doc = "Bit 1 - ACMPLP0 Window Function Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c0wde(&mut self) -> C0WDE_W<1> {
        C0WDE_W::new(self)
    }
    #[doc = "Bit 2 - ACMPLP0 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn c0vrf(&mut self) -> C0VRF_W<2> {
        C0VRF_W::new(self)
    }
    #[doc = "Bit 3 - ACMPLP0 Monitor Flag"]
    #[inline(always)]
    #[must_use]
    pub fn c0mon(&mut self) -> C0MON_W<3> {
        C0MON_W::new(self)
    }
    #[doc = "Bit 4 - ACMPLP1 Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1enb(&mut self) -> C1ENB_W<4> {
        C1ENB_W::new(self)
    }
    #[doc = "Bit 5 - ACMPLP1 Window Function Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1wde(&mut self) -> C1WDE_W<5> {
        C1WDE_W::new(self)
    }
    #[doc = "Bit 6 - ACMPLP1 Reference Voltage SelectionNote1: It's effective only at the time of standard mode.IVREF0 and IVREF1 are chosen at window mode condition in spite of setting of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn c1vrf(&mut self) -> C1VRF_W<6> {
        C1VRF_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP1 Monitor Flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1mon(&mut self) -> C1MON_W<7> {
        C1MON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMPLP Mode Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compmdr](index.html) module"]
pub struct COMPMDR_SPEC;
impl crate::RegisterSpec for COMPMDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [compmdr::R](R) reader structure"]
impl crate::Readable for COMPMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compmdr::W](W) writer structure"]
impl crate::Writable for COMPMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPMDR to value 0"]
impl crate::Resettable for COMPMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
