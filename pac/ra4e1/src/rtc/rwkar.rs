#[doc = "Register `RWKAR` reader"]
pub struct R(crate::R<RWKAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWKAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWKAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWKAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWKAR` writer"]
pub struct W(crate::W<RWKAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWKAR_SPEC>;
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
impl From<crate::W<RWKAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWKAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYW` reader - Day-of-Week Setting"]
pub type DAYW_R = crate::FieldReader<u8, DAYW_A>;
#[doc = "Day-of-Week Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAYW_A {
    #[doc = "0: Sunday"]
    _000 = 0,
    #[doc = "1: Monday"]
    _001 = 1,
    #[doc = "2: Tuesday"]
    _010 = 2,
    #[doc = "3: Wednesday"]
    _011 = 3,
    #[doc = "4: Thursday"]
    _100 = 4,
    #[doc = "5: Friday"]
    _101 = 5,
    #[doc = "6: Saturday"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<DAYW_A> for u8 {
    #[inline(always)]
    fn from(variant: DAYW_A) -> Self {
        variant as _
    }
}
impl DAYW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAYW_A {
        match self.bits {
            0 => DAYW_A::_000,
            1 => DAYW_A::_001,
            2 => DAYW_A::_010,
            3 => DAYW_A::_011,
            4 => DAYW_A::_100,
            5 => DAYW_A::_101,
            6 => DAYW_A::_110,
            7 => DAYW_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DAYW_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DAYW_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DAYW_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DAYW_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DAYW_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DAYW_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DAYW_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DAYW_A::_111
    }
}
#[doc = "Field `DAYW` writer - Day-of-Week Setting"]
pub type DAYW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RWKAR_SPEC, u8, DAYW_A, 3, O>;
impl<'a, const O: u8> DAYW_W<'a, O> {
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DAYW_A::_000)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DAYW_A::_001)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DAYW_A::_010)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DAYW_A::_011)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DAYW_A::_100)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DAYW_A::_101)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DAYW_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DAYW_A::_111)
    }
}
#[doc = "Field `ENB` reader - ENB"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "ENB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: Do not compare register value with RWKCNT counter value"]
    _0 = 0,
    #[doc = "1: Compare register value with RWKCNT counter value"]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - ENB"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, RWKAR_SPEC, ENB_A, O>;
impl<'a, const O: u8> ENB_W<'a, O> {
    #[doc = "Do not compare register value with RWKCNT counter value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENB_A::_0)
    }
    #[doc = "Compare register value with RWKCNT counter value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Day-of-Week Setting"]
    #[inline(always)]
    pub fn dayw(&self) -> DAYW_R {
        DAYW_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day-of-Week Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dayw(&mut self) -> DAYW_W<0> {
        DAYW_W::new(self)
    }
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<7> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Day-of-Week Alarm Register (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwkar](index.html) module"]
pub struct RWKAR_SPEC;
impl crate::RegisterSpec for RWKAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rwkar::R](R) reader structure"]
impl crate::Readable for RWKAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwkar::W](W) writer structure"]
impl crate::Writable for RWKAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWKAR to value 0"]
impl crate::Resettable for RWKAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
