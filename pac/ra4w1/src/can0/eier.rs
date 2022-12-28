#[doc = "Register `EIER` reader"]
pub struct R(crate::R<EIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIER` writer"]
pub struct W(crate::W<EIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIER_SPEC>;
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
impl From<crate::W<EIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable"]
pub type BEIE_R = crate::BitReader<BEIE_A>;
#[doc = "Bus Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIE_A {
    #[doc = "0: Bus error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus error interrupt enabled"]
    _1 = 1,
}
impl From<BEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIE_A {
        match self.bits {
            false => BEIE_A::_0,
            true => BEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEIE_A::_1
    }
}
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, BEIE_A, O>;
impl<'a, const O: u8> BEIE_W<'a, O> {
    #[doc = "Bus error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEIE_A::_0)
    }
    #[doc = "Bus error interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEIE_A::_1)
    }
}
#[doc = "Field `EWIE` reader - Error-Warning Interrupt Enable"]
pub type EWIE_R = crate::BitReader<EWIE_A>;
#[doc = "Error-Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIE_A {
    #[doc = "0: Error-warning interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error-warning interrupt enabled"]
    _1 = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIE_A {
        match self.bits {
            false => EWIE_A::_0,
            true => EWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWIE_A::_1
    }
}
#[doc = "Field `EWIE` writer - Error-Warning Interrupt Enable"]
pub type EWIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, EWIE_A, O>;
impl<'a, const O: u8> EWIE_W<'a, O> {
    #[doc = "Error-warning interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWIE_A::_0)
    }
    #[doc = "Error-warning interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWIE_A::_1)
    }
}
#[doc = "Field `EPIE` reader - Error-Passive Interrupt Enable"]
pub type EPIE_R = crate::BitReader<EPIE_A>;
#[doc = "Error-Passive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIE_A {
    #[doc = "0: Error-passive interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error-passive interrupt enabled"]
    _1 = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::_0,
            true => EPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIE_A::_1
    }
}
#[doc = "Field `EPIE` writer - Error-Passive Interrupt Enable"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, EPIE_A, O>;
impl<'a, const O: u8> EPIE_W<'a, O> {
    #[doc = "Error-passive interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPIE_A::_0)
    }
    #[doc = "Error-passive interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPIE_A::_1)
    }
}
#[doc = "Field `BOEIE` reader - Bus-Off Entry Interrupt Enable"]
pub type BOEIE_R = crate::BitReader<BOEIE_A>;
#[doc = "Bus-Off Entry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEIE_A {
    #[doc = "0: Bus-off entry interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off entry interrupt enabled"]
    _1 = 1,
}
impl From<BOEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOEIE_A {
        match self.bits {
            false => BOEIE_A::_0,
            true => BOEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEIE_A::_1
    }
}
#[doc = "Field `BOEIE` writer - Bus-Off Entry Interrupt Enable"]
pub type BOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, BOEIE_A, O>;
impl<'a, const O: u8> BOEIE_W<'a, O> {
    #[doc = "Bus-off entry interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOEIE_A::_0)
    }
    #[doc = "Bus-off entry interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOEIE_A::_1)
    }
}
#[doc = "Field `BORIE` reader - Bus-Off Recovery Interrupt Enable"]
pub type BORIE_R = crate::BitReader<BORIE_A>;
#[doc = "Bus-Off Recovery Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORIE_A {
    #[doc = "0: Bus-off recovery interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off recovery interrupt enabled"]
    _1 = 1,
}
impl From<BORIE_A> for bool {
    #[inline(always)]
    fn from(variant: BORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORIE_A {
        match self.bits {
            false => BORIE_A::_0,
            true => BORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORIE_A::_1
    }
}
#[doc = "Field `BORIE` writer - Bus-Off Recovery Interrupt Enable"]
pub type BORIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, BORIE_A, O>;
impl<'a, const O: u8> BORIE_W<'a, O> {
    #[doc = "Bus-off recovery interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BORIE_A::_0)
    }
    #[doc = "Bus-off recovery interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BORIE_A::_1)
    }
}
#[doc = "Field `ORIE` reader - Overrun Interrupt Enable"]
pub type ORIE_R = crate::BitReader<ORIE_A>;
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIE_A {
    #[doc = "0: Receive overrun interrupt disabled"]
    _0 = 0,
    #[doc = "1: Receive overrun interrupt enabled"]
    _1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::_0,
            true => ORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIE_A::_1
    }
}
#[doc = "Field `ORIE` writer - Overrun Interrupt Enable"]
pub type ORIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, ORIE_A, O>;
impl<'a, const O: u8> ORIE_W<'a, O> {
    #[doc = "Receive overrun interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORIE_A::_0)
    }
    #[doc = "Receive overrun interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORIE_A::_1)
    }
}
#[doc = "Field `OLIE` reader - Overload Frame Transmit Interrupt Enable"]
pub type OLIE_R = crate::BitReader<OLIE_A>;
#[doc = "Overload Frame Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLIE_A {
    #[doc = "0: Overload frame transmit interrupt disabled"]
    _0 = 0,
    #[doc = "1: Overload frame transmit interrupt enabled"]
    _1 = 1,
}
impl From<OLIE_A> for bool {
    #[inline(always)]
    fn from(variant: OLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OLIE_A {
        match self.bits {
            false => OLIE_A::_0,
            true => OLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLIE_A::_1
    }
}
#[doc = "Field `OLIE` writer - Overload Frame Transmit Interrupt Enable"]
pub type OLIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, OLIE_A, O>;
impl<'a, const O: u8> OLIE_W<'a, O> {
    #[doc = "Overload frame transmit interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OLIE_A::_0)
    }
    #[doc = "Overload frame transmit interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OLIE_A::_1)
    }
}
#[doc = "Field `BLIE` reader - Bus Lock Interrupt Enable"]
pub type BLIE_R = crate::BitReader<BLIE_A>;
#[doc = "Bus Lock Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLIE_A {
    #[doc = "0: Bus lock interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus lock interrupt enabled"]
    _1 = 1,
}
impl From<BLIE_A> for bool {
    #[inline(always)]
    fn from(variant: BLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLIE_A {
        match self.bits {
            false => BLIE_A::_0,
            true => BLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLIE_A::_1
    }
}
#[doc = "Field `BLIE` writer - Bus Lock Interrupt Enable"]
pub type BLIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIER_SPEC, BLIE_A, O>;
impl<'a, const O: u8> BLIE_W<'a, O> {
    #[doc = "Bus lock interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLIE_A::_0)
    }
    #[doc = "Bus lock interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(&self) -> BOEIE_R {
        BOEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(&self) -> BORIE_R {
        BORIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn olie(&self) -> OLIE_R {
        OLIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(&self) -> BLIE_R {
        BLIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<0> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 1 - Error-Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<1> {
        EWIE_W::new(self)
    }
    #[doc = "Bit 2 - Error-Passive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<2> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 3 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boeie(&mut self) -> BOEIE_W<3> {
        BOEIE_W::new(self)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn borie(&mut self) -> BORIE_W<4> {
        BORIE_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn orie(&mut self) -> ORIE_W<5> {
        ORIE_W::new(self)
    }
    #[doc = "Bit 6 - Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn olie(&mut self) -> OLIE_W<6> {
        OLIE_W::new(self)
    }
    #[doc = "Bit 7 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blie(&mut self) -> BLIE_W<7> {
        BLIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eier](index.html) module"]
pub struct EIER_SPEC;
impl crate::RegisterSpec for EIER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eier::R](R) reader structure"]
impl crate::Readable for EIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eier::W](W) writer structure"]
impl crate::Writable for EIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIER to value 0"]
impl crate::Resettable for EIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
